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

use std::{sync::Arc};
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde_yaml;
use anyhow::{Result, Context};
use log;
use tokio::sync::{Mutex, RwLock};
use tracing_subscriber::{layer::{self, SubscriberExt}, util::SubscriberInitExt};
use tracing::info;
use tower_sessions::{session_store::ExpiredDeletion, Expiry, Session, SessionManagerLayer};
use tower_sessions_sqlx_store::{sqlx::PgPool, PostgresStore};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config: Config = serde_yaml::from_reader(std::fs::File::open("config.yaml")?)?;
    let vocasync = Arc::new(Vocasync::new(config.clone()).await?);

    let session_store = PostgresStore::new(vocasync.pg_pool.clone());
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false);

    let app = Router::new()
        .route("/", get(welcome))
        .route("/api/currentPlay", get(get_current_play))
        .route("/api/playList", get(get_play_list))
        .route("/test/play", get(play_test))
        .route("/test/add", get(add_test))
        .with_state(vocasync)
        .layer(session_layer);
    let listener = tokio::net::TcpListener::bind(config.listen_address)
        .await?;
    
    info!("Welcome! Vocasync Start");
    info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;
    Ok(())
}
