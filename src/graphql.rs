use async_graphql::{
    Data, MergedObject, MergedSubscription, Response,
    http::{ALL_WEBSOCKET_PROTOCOLS, GraphiQLSource},
};
use async_graphql_axum::{GraphQLProtocol, GraphQLRequest, GraphQLResponse, GraphQLWebSocket};
use axum::{
    extract::{State, WebSocketUpgrade},
    http::{HeaderMap, header::AUTHORIZATION},
    response::{Html, IntoResponse, Response as AxumResponse},
};
use firebase_auth::FirebaseUser;
use serde::Deserialize;
use std::env;

use crate::AppState;

mod users_mutation;
mod users_query;
mod users_subscription;

#[derive(MergedObject, Default)]
pub struct QueryRoot(users_query::UsersQueryRoot);

#[derive(MergedObject, Default)]
pub struct MutationRoot(users_mutation::UsersMutationRoot);

#[derive(MergedSubscription, Default)]
pub struct SubscriptionRoot(users_subscription::UsersSubscriptionRoot);

pub async fn playground() -> impl IntoResponse {
    let mut playground = GraphiQLSource::build()
        .endpoint("/v1/graphql")
        .subscription_endpoint("/v1/ws");

    if cfg!(debug_assertions) {
        if let Ok(secret) = env::var("ADMIN_SECRET") {
            let leaked = Box::leak(secret.into_boxed_str());

            playground = playground
                .header("x-project-admin-secret", leaked)
                .ws_connection_param("x-project-admin-secret", leaked);
        }
    }

    Html(playground.finish())
}

pub async fn handler(
    State(AppState {
        firebase_auth,
        schema,
        ..
    }): State<AppState>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let auth = headers.get(AUTHORIZATION);
    let admin_secret = env::var("ADMIN_SECRET").ok();
    let project_admin_secret = headers
        .get("x-project-admin-secret")
        .and_then(|v| v.to_str().ok());

    if let Some(admin_secret) = admin_secret {
        if let Some(project_admin_secret) = project_admin_secret {
            if project_admin_secret == admin_secret {
                return schema.execute(req.into_inner()).await.into();
            }
        }
    }

    match auth {
        Some(auth) => {
            let auth_header = auth.to_str().unwrap();
            if auth_header.is_empty() {
                return GraphQLResponse::from(Response::new("Token is empty"));
            }

            let prefix_len = "Bearer ".len();
            if auth_header.len() <= prefix_len {
                return GraphQLResponse::from(Response::new("Token is empty"));
            }

            let token = auth_header[prefix_len..].to_string();
            match firebase_auth.verify::<FirebaseUser>(&token) {
                Ok(_) => schema.execute(req.into_inner()).await.into(),
                Err(_) => return GraphQLResponse::from(Response::new("Invalid token")),
            }
        }
        None => {
            return GraphQLResponse::from(Response::new("No Authorization header"));
        }
    }
}

#[derive(Deserialize, Debug)]
struct WebSocketAuthPayload {
    authorization: Option<String>,
    #[serde(rename = "x-project-admin-secret")]
    x_project_admin_secret: Option<String>,
}

pub async fn ws_handler(
    State(AppState {
        firebase_auth,
        schema,
        ..
    }): State<AppState>,
    protocol: GraphQLProtocol,
    websocket: WebSocketUpgrade,
) -> AxumResponse {
    websocket
        .protocols(ALL_WEBSOCKET_PROTOCOLS)
        .on_upgrade(move |stream| {
            GraphQLWebSocket::new(stream, schema.clone(), protocol)
                .on_connection_init(move |value| {
                    let firebase_auth = firebase_auth.clone();
                    async move {
                        if let Ok(WebSocketAuthPayload {
                            authorization,
                            x_project_admin_secret,
                        }) = simd_json::serde::from_borrowed_value::<WebSocketAuthPayload>(
                            value.try_into().unwrap(),
                        ) {
                            if let Some(x_project_admin_secret) = x_project_admin_secret {
                                if x_project_admin_secret == env::var("ADMIN_SECRET").ok().unwrap()
                                {
                                    tracing::info!("Admin connected");
                                    return Ok(Data::default());
                                }
                            }

                            if authorization.is_none() {
                                tracing::error!("Token is required");
                                return Err("Token is required".into());
                            }

                            let authorization = authorization.unwrap();
                            let authorization = authorization[7..].as_ref();

                            if firebase_auth.verify::<FirebaseUser>(authorization).is_ok() {
                                tracing::info!("User connected");
                                Ok(Data::default())
                            } else {
                                tracing::error!("Invalid token");
                                Err("Invalid token".into())
                            }
                        } else {
                            tracing::error!("Token is required");
                            Err("Token is required".into())
                        }
                    }
                })
                .serve()
        })
}
