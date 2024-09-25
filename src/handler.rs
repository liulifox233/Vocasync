use std::{str::FromStr, sync::Arc};
use crate::{
    error::Error, music::{self, CurrentMusic, Music, SerializeCurrentMusic, SerializePlayList}, user::User, vocasync::{self, Vocasync}
};
use axum::{
    extract::State, http::request, response::Response
};
use rand::Rng;
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

pub async fn add_test(State(vocasync): State<Arc<Vocasync>>) -> Result<String, Error> {
    let music = Music{
        uuid: uuid::Uuid::new_v4(),
        source: None,
        url: String::from_str("http://localhost:11451")?,
        url_timeout: None,
        cover: None,
        title: String::from_str("初音ミクの消失")?,
        album: None,
        artist: None,
        year: None,
        play_id: Some(uuid::Uuid::new_v4()),
        requester: None,
        duration: tokio::time::Duration::new(rand::thread_rng().gen_range(0..10), 0)
    };

    vocasync.room.add_music(music, uuid::Uuid::new_v4()).await?;
    let lock = vocasync.room.current_play.read().await;
    if lock.is_none() {
        drop(lock);
        spawn(async move {
            vocasync.room.play().await.unwrap();
        });
    }
    Ok("ok".to_string())
}

pub async fn get_play_list(State(vocasync): State<Arc<Vocasync>>) -> Result<axum::response::Json<SerializePlayList>, Error> {
    Ok(axum::Json(vocasync.room.get_play_list_serialize().await?))
}