use serde::{self,Serialize};
use uuid::Uuid;
use std::time::{self, Duration};
use crate::{
    source::{Source,self},
    user::User
};


#[derive(Clone, Serialize, Debug)]
pub struct PlayableMusic{
    pub music: Music,
    pub start_time: time::SystemTime
}

#[derive(Serialize, Debug)]
pub struct SerializePlayableMusic{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music: Option<Music>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u64>,
    pub is_play: bool
}

#[derive(Clone, Serialize, Debug)]
pub struct Music{
    pub uuid: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_timeout: Option<time::SystemTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<String>,
    #[serde(skip_serializing)]
    pub play_id: Option<Uuid>,
    #[serde(skip_serializing)]
    pub requester: Option<Uuid>,
    pub duration: Duration
}

#[derive(Serialize)]
pub struct SerializePlayList{
    pub total: u64,
    pub music_list: Vec<Music>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_now: Option<PlayableMusic>
}