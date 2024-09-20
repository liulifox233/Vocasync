use std::sync::Arc;
use crate::{
    error::VocasyncError, music::{CurrentMusic, SerializeCurrentMusic}, vocasync::Vocasync
};
use axum::{
    extract::State
};

pub async fn welcome(State(vocasync): State<Arc<Vocasync>>) -> Result<String, VocasyncError> {
    Ok("Welcome to Vocasync!".to_string())
}

pub async fn get_current_play(State(vocasync): State<Arc<Vocasync>>) -> Result<axum::response::Json<SerializeCurrentMusic>, VocasyncError> {
    Ok(axum::Json(vocasync.room.get_current_play_serialize().await?))
}

