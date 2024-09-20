mod vocasync;
mod config;
mod handler;
mod error;
mod user;
mod room;
mod music;
mod source;

use crate::{
    config::Config,
    vocasync::Vocasync,
    handler::*
};

use std::sync::Arc;
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde_yaml;
use anyhow::{Result, Context};
use log;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing::info;



#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config: Config = serde_yaml::from_reader(std::fs::File::open("config.yaml")?)?;
    let vocasync = Arc::new(Vocasync::new(config.clone()).await?);
    let app = Router::new()
        .route("/", get(welcome))
        .route("/api/currentPlay", get(get_current_play))
        .with_state(vocasync);
    let listener = tokio::net::TcpListener::bind(config.listen_address)
        .await?;
    
    info!("Welcome! Vocasync Start");
    info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;
    Ok(())
}
