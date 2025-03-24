use std::env;

use anyhow::Result;
use bb8::Pool as Bb8Pool;
use bb8_redis::RedisConnectionManager;

pub type RedisPool = Bb8Pool<RedisConnectionManager>;

pub async fn redis_pool() -> Result<RedisPool> {
    let manager = RedisConnectionManager::new(env::var("REDIS_HOST")?)?;
    let pool = RedisPool::builder().build(manager).await?;
    Ok(pool)
}
