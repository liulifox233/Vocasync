use crate::{
    music::{
        CurrentMusic, Music, SerializeCurrentMusic
    }, source::SourceKind, user::User
};
use std::{collections::LinkedList, str::FromStr};
use std::time;
use anyhow::Result;

#[derive(Clone)]
pub struct Room {
    pub play_now: bool,
    pub number: u32,
    pub userlist: Vec<User>,
    pub musiclist: LinkedList<Music>,
    pub last_time: time::SystemTime,
    pub last_person: String,
    pub current_play: Option<CurrentMusic>
}

impl Room {
    pub async fn new() -> Result<Self>{//暂时直接初始化，之后会加上从数据库加载数据的功能
        let play_now = false;
        let number = 0;
        let userlist = Vec::new();
        let musiclist = LinkedList::new();
        let last_time = time::SystemTime::now();
        let last_person = "最後の初音ミク".to_string();
        let music = Music{
            uuid: uuid::Uuid::new_v4(),
            source: None,
            url: String::from_str("http://localhost:114514")?,
            url_timeout: None,
            cover: None,
            name: String::from_str("初音ミクの消失")?,
            album: None,
            artist: None,
            years: None,
        };
        let current_play = Some(CurrentMusic{
            music,
            start_time: time::SystemTime::now(),
            requester: None
        });

        let res = Self {
            play_now,
            number,
            userlist,
            musiclist,
            last_time,
            last_person,
            current_play
        };

        Ok(res)
    }

    pub async fn get_current_play_serialize(&self) -> Result<SerializeCurrentMusic> {
        let current_music = self.current_play.clone();
        let position;
        let play_now;
        let music;
        match current_music {
            Some(m) => {
                play_now = true;
                position = Some(time::SystemTime::now().duration_since(m.start_time)?.as_secs());
                music = Some(m.music)
            },
            _ => {
                play_now = false;
                position = None;
                music = None;
            }
        };

        let res = SerializeCurrentMusic {
            music,
            position,
            play_now
        };
        Ok(res)
    }
}

