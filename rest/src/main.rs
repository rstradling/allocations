mod http;
use crate::http::{HttpServer, HttpServerConfig};
use std::env;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Error Loading the config from the environment")]
    LoadEnvError { name: String },
}

const DATABASE_URL_KEY: &str = "DATABASE_URL";

const SERVER_PORT_KEY: &str = "SERVER_PORT";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub server_port: String,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Config, ConfigError> {
        dotenvy::dotenv().ok();
        let server_port = load_env(SERVER_PORT_KEY)?;
        let database_url = load_env(DATABASE_URL_KEY)?;

        Ok(Config {
            server_port,
            database_url,
        })
    }
}

fn load_env(key: &str) -> Result<String, ConfigError> {
    env::var(key).map_err(|e| ConfigError::LoadEnvError {
        name: e.to_string(),
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;
    tracing_subscriber::fmt::init();

    let server_config = HttpServerConfig {
        port: &config.server_port,
    };
    let http_server = HttpServer::new(server_config).await?;
    http_server.run().await
}
