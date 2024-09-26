use std::{str::FromStr, sync::Arc};
use crate::{
    error::Error, music::{self, Music, PlayableMusic, SerializePlayList, SerializePlayableMusic}, source::MusicApi, user::User, vocasync::{self, Vocasync}
};
use axum::{
    extract::{Path, State}, http::request, response::Response
};
use rand::Rng;
use tokio::{spawn, sync::RwLock};

pub async fn welcome(State(vocasync): State<Arc<Vocasync>>) -> Result<String, Error> {
    Ok("Welcome to Vocasync!".to_string())
}

pub async fn get_current_play(State(vocasync): State<Arc<Vocasync>>) -> Result<axum::response::Json<SerializePlayableMusic>, Error> {
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
        url: Some(String::from_str("http://localhost:11451")?),
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

pub async fn search_user_test(State(vocasync): State<Arc<Vocasync>>, Path(name): Path<String>) -> Result<axum::response::Json<String>, Error> {
    Ok(axum::Json(format!("{:#?}",vocasync.neteaseapi.search_user(name).await)))
}

pub async fn get_music_by_id_test(State(vocasync): State<Arc<Vocasync>>, Path(id): Path<String>) -> Result<String, Error> {
    Ok(format!("{:#?}",vocasync.neteaseapi.get_music_by_id(id).await))
}

pub async fn get_user_playlist_test(State(vocasync): State<Arc<Vocasync>>, Path(id): Path<String>) -> Result<String, Error> {
    Ok(format!("{:#?}",vocasync.neteaseapi.get_user_playlist(id).await))
}

pub async fn get_music_by_playlist_test(State(vocasync): State<Arc<Vocasync>>, Path(id): Path<String>) -> Result<String, Error> {
    Ok(format!("{:#?}",vocasync.neteaseapi.get_music_by_playlist(id, 0).await))
}