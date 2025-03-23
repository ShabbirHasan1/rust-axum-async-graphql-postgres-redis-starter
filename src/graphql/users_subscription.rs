use crate::types::users::User;
use async_graphql::{Context, FieldResult, Subscription};

use std::{pin::Pin, time::Duration};
use tokio::{sync::mpsc, time::interval};
use tokio_stream::{Stream, wrappers::ReceiverStream};

#[derive(Default)]
pub struct UsersSubscriptionRoot;

#[Subscription(rename_fields = "snake_case", rename_args = "snake_case")]
impl UsersSubscriptionRoot {
    async fn select_user_by_id<'a>(
        &self,
        _context: &Context<'a>,
        _id: String,
    ) -> Pin<Box<dyn Stream<Item = FieldResult<User>> + Send + '_>> {
        let (tx, rx) = mpsc::channel::<FieldResult<User>>(20);

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(1));

            loop {
                interval.tick().await;
                let _ = tx
                    .send(Ok(User {
                        id: 0,
                        email: String::from("user@example.com"),
                    }))
                    .await;
            }
        });

        Box::pin(ReceiverStream::new(rx))
    }
}
