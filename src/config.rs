use serde::Deserialize;
use anyhow::Result;


fn default_listen_address() -> String {
    "http://0.0.0.0:3939".to_owned()
}

fn default_frontend_url() -> String {
    "http://localhost:3000".to_owned()
}

#[derive(Deserialize,Clone)]
pub struct NeteaseConfig {
    pub url: String,
    pub phone_num: String,
    pub password: String,
}

#[derive(Deserialize,Clone)]
pub struct Config {
    #[serde(default = "default_listen_address")]
    pub listen_address: String,
    #[serde(default = "default_frontend_url")]
    pub frontend_url: String,
    pub database_url: String,

    pub neteaseapi: NeteaseConfig,
}

impl Config {
    pub async fn check(&self) -> Result<()> {
        //TODO
        Ok(())
    }
}
