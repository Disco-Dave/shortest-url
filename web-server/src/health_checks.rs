use axum::Router;
use http::StatusCode;
use sqlx::PgPool;

async fn handle_get() -> StatusCode {
    tracing::info!("test 123?");
    StatusCode::NO_CONTENT
}

async fn handle_get_database(db_pool: PgPool) -> Result<StatusCode, StatusCode> {
    let row = sqlx::query!("SELECT version();")
        .fetch_one(&db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Health check query to the database failed: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if row.version.is_some() {
        Ok(StatusCode::NO_CONTENT)
    } else {
        tracing::error!("Health check query to database returned no results.");
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub fn router(db_pool: PgPool) -> Router {
    use axum::routing::get;

    Router::new().route("/", get(handle_get)).route(
        "/databases",
        get({
            let db_pool = db_pool.clone();
            move || handle_get_database(db_pool)
        }),
    )
}
