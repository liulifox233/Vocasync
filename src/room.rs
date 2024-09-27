use std::time;
use anyhow::{Ok, Result};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::music::SerializePlayList;
use crate::{
    music::{
        PlayableMusic, Music, SerializePlayableMusic
    }, 
    user::User
};

pub struct Room {
    pub userlist: Vec<User>,
    pub musiclist: RwLock<Vec<Music>>,
    pub last_time: time::SystemTime,
    pub last_person: String,
    pub current_play: RwLock<Option<PlayableMusic>>,
}

impl Room {
    pub async fn new() -> Result<Self>{//暂时直接初始化，之后会加上从数据库加载数据的功能
        let userlist = Vec::new();
        let musiclist = Vec::new();
        let last_time = time::SystemTime::now();
        let last_person = "最後の初音ミク".to_string();
        let musiclist = RwLock::new(musiclist);
        let current_play = RwLock::new(None);

        let res = Self {
            userlist,
            musiclist,
            last_time,
            last_person,
            current_play,
        };

        Ok(res)
    }

    pub async fn get_play_list_serialize(&self) -> Result<SerializePlayList> {
        let music_list = self.musiclist.read().await.to_owned();
        let play_now = self.current_play.read().await.to_owned();
        let total = music_list.len().try_into()?;
        
        let res = SerializePlayList {
            total,
            music_list,
            play_now
        };
        Ok(res)
    }

    pub async fn get_current_play_serialize(&self) -> Result<SerializePlayableMusic> {
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

        let res = SerializePlayableMusic {
            music,
            position,
            is_play
        };
        Ok(res)
    }

    pub async fn add_music(&self, mut music: Music, requester: Uuid) -> Result<()> {
        music.play_id = Some(uuid::Uuid::new_v4());
        music.requester = Some(requester);
        self.musiclist.write().await.insert(0, music);
        Ok(())
    }

    pub async fn move_top(&mut self, music: Music) -> Result<()> {
        let mut musiclist = self.musiclist.write().await;
        let mut it = musiclist.iter_mut();
        match it.position(|m| m.play_id == music.play_id) {
            Some(p) => {
                let removed_music = musiclist.remove(p);
                musiclist.push(removed_music);
                Ok(())
            },
            None => Err(anyhow::anyhow!("Can not find music"))
        }
    }

    pub async fn play(&self) -> Result<()> { // 既可以用作播放也可用作切歌
        loop {
            let mut musiclist_write = self.musiclist.write().await;
            if let Some(next_play) = musiclist_write.pop() {
                drop(musiclist_write);
                let now_play = Some(PlayableMusic {
                    music: next_play.clone(),
                    start_time: std::time::SystemTime::now(),
                });
                let duration = next_play.duration.to_owned();
                let mut current_play_write = self.current_play.write().await;
                *current_play_write = now_play.clone();
                drop(current_play_write);
                tokio::time::sleep(duration + std::time::Duration::new(1, 0)).await;
                if let (Some(before), Some(now)) = (self.current_play.read().await.clone(), now_play) {
                    if before.music.play_id != now.music.play_id {
                        return Ok(());
                    }
                }
            } else {
                let mut current_play = self.current_play.write().await;
                *current_play = None;
                return Ok(());
            }
        }
    }

}

