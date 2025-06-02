use anyhow::Result;
use async_graphql::{Schema, dataloader::DataLoader};
use axum::{Router, http::Method, routing::get};
use dotenvy::dotenv;
use firebase_auth::{FirebaseAuth, FirebaseAuthState};
use rust_axum_async_graphql_postgres_redis_starter::{
    AppState,
    dataloaders::users,
    graphql::{self, MutationRoot, QueryRoot, SubscriptionRoot},
    postgres, redis,
};
use std::env;
use tokio::net::TcpListener;
use tower_http::{
    compression::CompressionLayer,
    cors::{AllowOrigin, CorsLayer},
    trace::TraceLayer,
};
use tracing::Level;

#[cfg(debug_assertions)]
use async_graphql::extensions::ApolloTracing;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;

    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_max_level(Level::INFO)
        .init();

    let postgres = postgres::pgpool().await?;
    let redis = redis::redis_pool().await?;
    let firebase_auth = FirebaseAuth::new(&env::var("FIREBASE_PROJECT_ID")?).await;
    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        SubscriptionRoot::default(),
    );
    #[cfg(debug_assertions)]
    let schema = schema.extension(ApolloTracing);
    let schema = schema
        .data(postgres.clone())
        .data(redis.clone())
        .data(DataLoader::new(
            users::DataLoader::new(postgres, redis),
            tokio::spawn,
        ))
        .finish();

    let app = Router::new()
        .route(
            "/v1/graphql",
            get(graphql::playground).post(graphql::handler),
        )
        .route("/v1/ws", get(graphql::ws_handler))
        .with_state(AppState {
            auth: FirebaseAuthState::new(firebase_auth),
            schema,
        })
        .layer(CompressionLayer::new().gzip(true))
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::predicate(|_, _| true))
                .allow_methods([Method::GET, Method::POST]),
        );

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    tracing::info!("GrahpiQL: http://0.0.0.0:8080/v1/graphql");
    axum::serve(listener, app).await?;

    Ok(())
}
