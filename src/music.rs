use serde::{self,Serialize};
use uuid::Uuid;
use std::time;
use crate::{
    source::{Source,self},
    user::User
};


#[derive(Clone, Serialize)]
pub struct CurrentMusic{
    pub music: Music,
    pub start_time: time::SystemTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester: Option<User>
}

#[derive(Serialize)]
pub struct SerializeCurrentMusic{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music: Option<Music>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u64>,
    pub play_now: bool
}

#[derive(Clone, Serialize)]
pub struct Music{
    pub uuid: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_timeout: Option<time::SystemTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub years: Option<String>
}