use crate::health_checks;
use crate::urls;

use anyhow::Context;
use axum::body::Body;
use http::Request;
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use uuid::Uuid;

pub async fn start(socket_address: &SocketAddr, db_pool: PgPool) -> anyhow::Result<()> {
    let router = axum::Router::new()
        .layer(
            TraceLayer::new_for_http().make_span_with(|_request: &Request<Body>| {
                let request_id = Uuid::new_v4();
                tracing::info_span!("request", id = ?request_id)
            }),
        )
        .merge(urls::router(db_pool.clone()))
        .nest("/health-checks", health_checks::router(db_pool.clone()));

    tracing::info!("Server now listening on port {}", socket_address.port());

    axum::Server::bind(socket_address)
        .serve(router.into_make_service())
        .await
        .context(format!(
            "Unable to bind server to {:?}. Is this port already in use?",
            socket_address
        ))
}
