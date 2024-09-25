use std::sync::Arc;
use crate::{
    error::Error, music::{CurrentMusic, SerializeCurrentMusic}, vocasync::{self, Vocasync}
};
use axum::{
    extract::State, response::Response
};
use tokio::{spawn, sync::RwLock};

pub async fn welcome(State(vocasync): State<Arc<Vocasync>>) -> Result<String, Error> {
    Ok("Welcome to Vocasync!".to_string())
}

pub async fn get_current_play(State(vocasync): State<Arc<Vocasync>>) -> Result<axum::response::Json<SerializeCurrentMusic>, Error> {
    Ok(axum::Json(vocasync.room.get_current_play_serialize().await?))
}

pub async fn play_test(State(vocasync): State<Arc<Vocasync>>) -> Result<String, Error> {
    spawn(async move {
        vocasync.room.play().await.unwrap();
    });
    Ok("ok".to_string())
}