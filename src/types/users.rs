use async_graphql::Object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct User {
    pub id: i64,
    pub email: String,
}

#[Object]
impl User {
    async fn id(&self) -> i64 {
        self.id
    }

    async fn email(&self) -> &str {
        &self.email
    }
}
