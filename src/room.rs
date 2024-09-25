use std::{str::FromStr, time::Duration};
use std::time;
use anyhow::{Ok, Result};
use tokio::sync::RwLock;
use tokio::time::sleep;
use uuid::Uuid;

use crate::{
    music::{
        CurrentMusic, Music, SerializeCurrentMusic
    }, 
    source::SourceKind, 
    user::User
};

pub struct Room {
    pub userlist: Vec<User>,
    pub musiclist: RwLock<Vec<Music>>,
    pub last_time: time::SystemTime,
    pub last_person: String,
    pub current_play: RwLock<Option<CurrentMusic>>,
}

impl Room {
    pub async fn new() -> Result<Self>{//暂时直接初始化，之后会加上从数据库加载数据的功能
        let userlist = Vec::new();
        let mut musiclist = Vec::new();
        let last_time = time::SystemTime::now();
        let last_person = "最後の初音ミク".to_string();
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
            duration: Duration::new(10, 0)
        };
        musiclist.push(music.clone());
        let current_play = RwLock::new(Some(CurrentMusic{
            music,
            start_time: time::SystemTime::now()
        }));
        let musiclist = RwLock::new(musiclist);
        // let current_play = None;

        let res = Self {
            userlist,
            musiclist,
            last_time,
            last_person,
            current_play,
        };

        Ok(res)
    }

    pub async fn get_current_play_serialize(&self) -> Result<SerializeCurrentMusic> {
        let current_music = self.current_play.read().await.clone();
        let position;
        let is_play;
        let music;
        match current_music {
            Some(m) => {
                is_play = true;
                position = Some(time::SystemTime::now().duration_since(m.start_time)?.as_secs());
                music = Some(m.music)
            },
            None => {
                is_play = false;
                position = None;
                music = None;
            }
        };

        let res = SerializeCurrentMusic {
            music,
            position,
            is_play
        };
        Ok(res)
    }

    pub async fn add_music(&mut self, mut music: Music, requester: Uuid) -> Result<()> {
        music.play_id = Some(uuid::Uuid::new_v4());
        music.requester = Some(requester);
        self.musiclist.write().await.insert(0, music);
        match *self.current_play.read().await {
            None => {self.play().await?},
            _ => ()
        }
        Ok(())
    }

    pub async fn move_top(&mut self, music: Music) -> Result<()> {
        let mut musiclist = self.musiclist.write().await;
        let mut it = musiclist.iter_mut();
        match it.position(|m| m.play_id == music.play_id) {
            Some(p) => {
                let removed_music = musiclist.remove(p);
                musiclist.insert(0,removed_music);
                Ok(())
            },
            None => Err(anyhow::anyhow!("Can not find music"))
        }
    }

    pub async fn play(&self) -> Result<()> {//既可以用作播放也可用作切歌
        loop {
            if let Some(next_play) = self.musiclist.write().await.pop() {
                let current_play = Some(CurrentMusic {
                    music: next_play.clone(),
                    start_time: std::time::SystemTime::now(),
                });
                *self.current_play.write().await = current_play.clone();
                sleep(next_play.duration + Duration::new(1, 0)).await;
                if let (Some(before), Some(now)) = (self.current_play.read().await.clone(), current_play) {
                    if before.music.play_id != now.music.play_id {
                        return Ok(());
                    } 
                }
            }else {
                *self.current_play.write().await = None;
                return Ok(());
            }
        }
    }

}

