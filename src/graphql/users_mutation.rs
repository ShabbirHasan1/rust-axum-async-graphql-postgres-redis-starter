use async_graphql::{FieldResult, Object};

use crate::types::users::User;

#[derive(Default, Debug, Clone, Copy)]
pub struct UsersMutationRoot;

#[Object(rename_fields = "snake_case", rename_args = "snake_case")]
impl UsersMutationRoot {
    async fn update_user(&self, user_id: i64, email: String) -> FieldResult<User> {
        Ok(User { id: user_id, email })
    }
}
