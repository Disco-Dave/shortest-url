use crate::health_checks;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;

pub async fn start(socket_address: &SocketAddr) {
    let subscriber = tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer());

    tracing::subscriber::set_global_default(subscriber).expect("Unable to set logger");

    let router = axum::Router::new()
        .layer(TraceLayer::new_for_http())
        .nest("/health-checks", health_checks::router());

    tracing::info!("Server now listening on port {}", socket_address.port());

    axum::Server::bind(socket_address)
        .serve(router.into_make_service())
        .await
        .expect(&format!(
            "Unable to bind server to {:?}. Is this port already in use?",
            socket_address
        ));
}
