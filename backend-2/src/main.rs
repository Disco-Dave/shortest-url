use anyhow::Context;
use shortest_url::config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_environment()
        .context("Unable to read configuration from environment variables.")?;

    shortest_url::start(config).await;

    Ok(())
}
