use std::path::PathBuf;
use logw::tracing::info;
use serde::{Deserialize, Serialize};
use api_sebba_dev_base::util;

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub service_ip: String,
    pub service_port: u16,
    pub allowed_origins: Vec<String>,
    pub database_url: String,
    pub database_max_connections: u32,
    pub management_api_key: String,
    pub sentry_dsn: Option<String>
}

fn config_dir() -> PathBuf {
    let dir: PathBuf = std::env::current_dir().expect("Failed to get current directory");

    let config_dir: String = std::env::var("SERVICE_CONFIG_DIR").unwrap_or_else(|_| String::from(".config"));

    dir.join(config_dir)
}

pub fn load() -> Configuration {
    let dir: PathBuf = config_dir();
    let config_path: PathBuf = dir.join("config.json");

    info!("Loading configuration from {}", &config_path.display());
    util::file::json(&config_path).expect("Failed to parse config.json")
}