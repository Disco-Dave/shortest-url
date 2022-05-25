use axum::Router;
use http::StatusCode;

#[tracing::instrument(name = "health check")]
async fn handle_get() -> StatusCode {
    tracing::info!("test");

    StatusCode::NO_CONTENT
}

pub fn router() -> Router {
    use axum::routing::get;

    Router::new().route("/", get(handle_get))
}
