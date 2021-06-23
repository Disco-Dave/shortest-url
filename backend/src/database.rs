use sqlx::PgPool;

use crate::settings::DatabaseSettings;

pub mod urls;

pub async fn get_pool(settings: &DatabaseSettings) -> PgPool {
    PgPool::connect(&settings.app_conn_string())
        .await
        .expect("Unable to connect to database")
}
