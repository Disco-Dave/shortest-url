mod base62;
pub mod config;
mod health_checks;
mod http;
mod logging;
mod urls;

use config::Config;

use sqlx::PgPool;

pub async fn start(config: Config) -> anyhow::Result<()> {
    logging::init(&config.trace_directive, config.trace_use_json)?;

    let pg_pool = PgPool::connect(&config.database_url).await?;

    http::start(&config.http_socket_address(), pg_pool.clone()).await?;

    pg_pool.close().await;

    Ok(())
}
