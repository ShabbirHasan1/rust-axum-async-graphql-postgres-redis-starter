use anyhow::{Ok, Result};
use bb8::Pool as Bb8Pool;
use bb8_postgres::PostgresConnectionManager;
use std::env;
use tokio_postgres::{Config, NoTls};

pub type PgPool = Bb8Pool<PostgresConnectionManager<NoTls>>;

pub async fn pgpool() -> Result<PgPool> {
    let mut config = Config::new();
    config.host(env::var("POSTGRES_HOST")?);
    config.password(env::var("POSTGRES_PASSWORD")?);
    config.dbname(env::var("POSTGRES_DB")?);
    config.user(env::var("POSTGRES_USER")?);
    config.port(env::var("POSTGRES_PORT")?.parse()?);

    // TODO: add ssl configuration later
    let manager = PostgresConnectionManager::new(config, NoTls);
    let pool = PgPool::builder().max_size(50).build(manager).await?;
    Ok(pool)
}
