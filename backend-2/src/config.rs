use anyhow::Context;
use serde::Deserialize;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct Config {
    pub http_port: u16,
}

impl Config {
    pub fn http_socket_address(&self) -> SocketAddr {
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, self.http_port))
    }

    pub fn from_environment() -> anyhow::Result<Config> {
        dotenvy::dotenv().ok();

        let config = config::Config::builder();

        let parsed_config = config
            .add_source(
                config::Environment::default()
                    .prefix("SHORTEST_URL")
                    .prefix_separator("_"),
            )
            .build()
            .context("Unable to build configuration.")?
            .try_deserialize()
            .context("Unable to parse configuration.")?;

        Ok(parsed_config)
    }
}
