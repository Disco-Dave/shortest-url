use shortest_url::{logging, settings::Settings};

#[tokio::main]
async fn main() {
    let settings = Settings::new().expect("Failed to get application settings.");
    let _loging_guard = logging::init(&settings.log);
    let (addr, server) = shortest_url::start(&settings.http, &settings.database).await;
    tracing::info!("Now listening on: {}", addr);
    server.await
}
