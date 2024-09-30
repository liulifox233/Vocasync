mod vocasync;
mod config;
mod handler;
mod error;
mod user;
mod room;
mod music;
mod source;
mod api;

use crate::{
    config::Config,
    vocasync::Vocasync,
    handler::*
};

use std::sync::Arc;
use axum::{
    routing::get,
    Router,
};
use serde_yaml;
use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing::info;
use tower_sessions::SessionManagerLayer;
use tower_sessions_sqlx_store::PostgresStore;

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

    let proxy_router = Router::new()
        .route("/*path", get(proxy))
        .route("/", get(proxy_root));

    let app = Router::new()
        .route("/api/currentPlay", get(get_current_play))
        .route("/api/playList", get(get_play_list))
        .route("/api/add/:source/:id", get(add_music_to_playlist))
        .nest("/", proxy_router)
        .route("/test/play", get(play_test))
        .route("/test/add", get(add_test))
        .route("/test/search/:name", get(search_user_test))
        .route("/test/music_id/:id", get(get_music_by_id_test))
        .route("/test/get_user_playlist/:id", get(get_user_playlist_test))
        .route("/test/get_music_by_playlist/:id", get(get_music_by_playlist_test))
        .with_state(vocasync)
        .layer(session_layer);
    let listener = tokio::net::TcpListener::bind(config.listen_address)
        .await?;
    
    info!("Welcome! Vocasync Start");
    info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test() {
        let config: Config = serde_yaml::from_reader(std::fs::File::open("config.yaml").unwrap()).unwrap();
        let vocasync = Arc::new(Vocasync::new(config.clone()).await.unwrap());

        let session_store = PostgresStore::new(vocasync.pg_pool.clone());
        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false);

            let app = Router::new()
            .route("/api/currentPlay", get(get_current_play))
            .route("/api/playList", get(get_play_list))
            .route("/add/:source/:id", get(add_music_to_playlist))
            .route("/test/play", get(play_test))
            .route("/test/add", get(add_test))
            .route("/test/search/:name", get(search_user_test))
            .route("/test/music_id/:id", get(get_music_by_id_test))
            .route("/test/get_user_playlist/:id", get(get_user_playlist_test))
            .route("/test/get_music_by_playlist/:id", get(get_music_by_playlist_test))
            .with_state(vocasync)
            .layer(session_layer);
        let listener = tokio::net::TcpListener::bind(config.listen_address.clone())
            .await.unwrap();
        
        info!("Welcome! Vocasync Start");
        info!("listening on {}", listener.local_addr().unwrap());
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        reqwest::get(
            format!(
                "{}/test/add", 
                config.listen_address.clone(),
            )
        ).await.unwrap();

        reqwest::get(
            format!(
                "{}/add/Netease/1308258153", 
                config.listen_address.clone(),
            )
        ).await.unwrap();

        reqwest::get(
            format!(
                "{}/api/currentPlay", 
                config.listen_address.clone(),
            )
        ).await.unwrap();

        reqwest::get(
            format!(
                "{}/api/playList", 
                config.listen_address.clone(),
            )
        ).await.unwrap();
    }
}