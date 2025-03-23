use crate::types::users::User;
use crate::{postgres::PgPool, redis::RedisPool};
use async_graphql::dataloader::Loader;

use std::{collections::HashMap, sync::Arc};

pub struct DataLoader {
    pool: PgPool,
    cache: RedisPool,
}

impl DataLoader {
    pub fn new(pool: PgPool, cache: RedisPool) -> Self {
        Self { pool, cache }
    }
}

impl Loader<String> for DataLoader {
    type Value = User;
    type Error = Arc<anyhow::Error>;

    async fn load(&self, _keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let users = HashMap::<String, User>::default();
        Ok(users)
    }
}
