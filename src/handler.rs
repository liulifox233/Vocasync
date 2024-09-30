use std::{str::FromStr, sync::Arc};
use crate::{
    error::Error, music::{Music, SerializePlayList, SerializePlayableMusic}, source::{self, MusicApi}, vocasync::Vocasync
};
use axum::extract::{Path, State};
use rand::Rng;
use serde_json::Value;
use tokio::spawn;


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

pub async fn get_play_list(
    State(vocasync): State<Arc<Vocasync>>,
) -> Result<axum::response::Json<SerializePlayList>, Error> {
    Ok(axum::Json(vocasync.room.get_play_list_serialize().await?))
}

pub async fn search_user_test(
    State(vocasync): State<Arc<Vocasync>>, 
    Path(name): Path<String>
) -> Result<axum::response::Json<String>, Error> {
    Ok(axum::Json(format!("{:#?}",vocasync.neteaseapi.search_user(name).await)))
}

pub async fn get_music_by_id_test(
    State(vocasync): State<Arc<Vocasync>>, 
    Path(id): Path<String>,
) -> Result<String, Error> {
    Ok(format!("{:#?}",vocasync.neteaseapi.get_music_by_id(id).await))
}

pub async fn get_user_playlist_test(
    State(vocasync): State<Arc<Vocasync>>, 
    Path(id): Path<String>
) -> Result<String, Error> {
    Ok(format!("{:#?}",vocasync.neteaseapi.get_user_playlist(id).await))
}

pub async fn get_music_by_playlist_test(
    State(vocasync): State<Arc<Vocasync>>, 
    Path(id): Path<String>
) -> Result<String, Error> {
    Ok(format!("{:#?}",vocasync.neteaseapi.get_music_by_playlist(id, 0).await))
}

pub async fn add_music_to_playlist(
    State(vocasync): State<Arc<Vocasync>>,
    Path((source, id)): Path<(source::SourceKind, String)>
) -> Result<axum::response::Json<Value>, Error> {
    let music = match source {
        source::SourceKind::Netease => vocasync.neteaseapi.get_music_by_id(id).await?,
        source::SourceKind::Applemusic => vocasync.neteaseapi.get_music_by_id(id).await?,
        source::SourceKind::Other => vocasync.neteaseapi.get_music_by_id(id).await?
    };
    vocasync.room.add_music(music, uuid::Uuid::new_v4()).await?;
    let lock = vocasync.room.current_play.read().await;
    if lock.is_none() {
        drop(lock);
        spawn(async move {
            vocasync.room.play().await.unwrap();
        });
    }
    Ok(axum::Json(serde_json::json!({ "code": 200 })))
}

pub async fn proxy(
    State(vocasync): State<Arc<Vocasync>>,
    Path(path): Path<String>,
) -> Result<axum::response::Response<axum::body::Body>, Error> {
    let uri = format!("{}/{}", vocasync.config.frontend_url, path);
    let res = vocasync.client.get(&uri).send().await?;

    let status = res.status();
    let headers = res.headers().clone();
    let body_bytes = res.bytes().await?;

    let mut response_builder = axum::response::Response::builder().status(status);
    for (key, value) in headers.iter() {
        response_builder = response_builder.header(key, value);
    }

    let response = response_builder
        .body(axum::body::Body::from(body_bytes))
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    Ok(response)
}


pub async fn proxy_root(
    State(vocasync): State<Arc<Vocasync>>,
) -> Result<axum::response::Response<axum::body::Body>, Error> {
    let uri = format!("{}/", vocasync.config.frontend_url);
    let res = vocasync.client.get(&uri).send().await?;

    let status = res.status();
    let headers = res.headers().clone();
    let body_bytes = res.bytes().await?;

    let mut response_builder = axum::response::Response::builder().status(status);
    for (key, value) in headers.iter() {
        response_builder = response_builder.header(key, value);
    }

    let response = response_builder
        .body(axum::body::Body::from(body_bytes))
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    Ok(response)
}