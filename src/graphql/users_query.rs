use async_graphql::{Context, FieldResult, Object};

use crate::types::users::User;

#[derive(Default)]
pub struct UsersQueryRoot;

#[Object(rename_fields = "snake_case", rename_args = "snake_case")]
impl UsersQueryRoot {
    async fn select_users(
        &self,
        _context: &Context<'_>,
        _limit: Option<i64>,
        _offset: Option<i64>,
    ) -> FieldResult<Vec<User>> {
        Ok(vec![User {
            id: 0,
            email: String::from("user@example.com"),
        }])
    }
}
