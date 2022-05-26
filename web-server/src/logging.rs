use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Layer};

pub fn init(directive: &str, use_json: bool) -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::registry()
        .with(EnvFilter::new(directive))
        .with(if use_json {
            fmt::layer().json().boxed()
        } else {
            fmt::layer().pretty().boxed()
        });

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}
