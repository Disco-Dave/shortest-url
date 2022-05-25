pub mod config;
mod health_checks;
mod http;

use crate::config::Config;

pub async fn start(config: Config) {
    http::start(&config.http_socket_address()).await
}
