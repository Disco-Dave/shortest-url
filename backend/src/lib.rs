use std::net::SocketAddr;

use settings::DatabaseSettings;
use settings::HttpSettings;
use warp::Future;

mod actions;
mod base62;
mod database;
mod http;
pub mod logging;
pub mod settings;

pub async fn start(
    http_settings: &HttpSettings,
    database_settings: &DatabaseSettings,
) -> (SocketAddr, impl Future<Output = ()> + 'static) {
    let db_pool = database::get_pool(database_settings).await;
    crate::http::start(http_settings, db_pool).await
}
