use axum::http::status;
use serde::{Deserialize, Serialize};

use crate::{music::Music, user::User};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum SourceKind{
    Netease,
    Applemusic,
    Other
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct  Source {
    pub id: String,
    pub kind: SourceKind
}

#[derive(Debug)]
pub struct PlayList {
    pub source: Source,
    pub name: String,
}

pub trait MusicApi {
    async fn login(&mut self) -> anyhow::Result<()>;
    async fn get_music_by_id(&self, id: String) -> anyhow::Result<Music>;
    async fn search_user(&self, name: String) -> anyhow::Result<Vec<User>>;
    async fn get_user_playlist(&self, id: String) -> anyhow::Result<Vec<PlayList>>;
    async fn get_music_by_playlist(&self, id: String, offset: u64) -> anyhow::Result<Vec<Music>>;
}
