use serde::Deserialize;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

fn default_http_port() -> u16 {
    4000
}
fn default_trace_directive() -> String {
    String::from("trace")
}
fn default_trace_use_json() -> bool {
    true
}
fn default_database_url() -> String {
    String::from("")
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "default_http_port")]
    pub http_port: u16,

    #[serde(default = "default_trace_directive")]
    pub trace_directive: String,

    #[serde(default = "default_trace_use_json")]
    pub trace_use_json: bool,

    #[serde(default = "default_database_url")]
    pub database_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            http_port: default_http_port(),
            trace_directive: default_trace_directive(),
            trace_use_json: default_trace_use_json(),
            database_url: default_database_url(),
        }
    }
}

impl Config {
    pub fn http_socket_address(&self) -> SocketAddr {
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, self.http_port))
    }

    pub fn from_environment() -> anyhow::Result<Config> {
        let config = envy::prefixed("SHORTEST_URL_").from_env()?;
        Ok(config)
    }
}
