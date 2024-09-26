use crate::{music::{self, Music}, source::{MusicApi, PlayList, Source}, user::User};
use anyhow::Result;
use axum::{extract::rejection::FailedToBufferBody, response, Json};
use serde_json::{json, Value};
use tower_sessions::cookie::Cookie;
use tracing::info;
use uuid::{uuid, Uuid};
use core::time;
use std::{fs, time::Duration};
use std::time::SystemTime;

use crate::{
    source
};


pub struct NeteaseApi {
    pub url: String,
    pub phone_num: String,
    pub password: String,
    pub cookie: String,
}

impl NeteaseApi {
    async fn check_cookie(&self, cookie: Option<String>) -> Result<()> {
        if cookie.is_none() {return Err(anyhow::anyhow!("Please check your cookie.txt"));}
        let res = reqwest::get(
            format!(
                "{}/user/account?realIP=183.232.239.22&timestamp={}&cookie={}", 
                self.url.clone(),
                SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs(),
                cookie.unwrap().clone()
            )).await?;
        let res = res.json::<Value>().await?;
        if res.get("profile").unwrap().is_null() {
            return Err(anyhow::anyhow!("Please check your cookie.txt"));
        }
        Ok(())
    }
    
    async fn parse_music_url(&self, mut music: Music) -> Result<Music> {
        let res = reqwest::get(
            format!(
                "{}/song/url?realIP=183.232.239.22&id={}&cookie={}", 
                self.url.clone(),
                music.source.clone().unwrap().id,
                self.cookie.clone()
            )).await?;
        let res = res.json::<Value>().await?;
        if res.get("code").unwrap().as_u64().unwrap() != 200 {
            return Err(anyhow::anyhow!("Get song failed!"));
        }else {
            music.url = Some(res.get("data").unwrap().as_array().unwrap().get(0).unwrap().get("url").unwrap().to_string());
        }
        Ok(music)
    }

    async fn parse_music(&self, res: &Value) -> Result<Vec<Music>> {
        let mut music_list = Vec::new();
        let res = res.get("songs").unwrap().as_array().unwrap();
        for music in res.iter() {
            let uuid = Uuid::new_v4();
            let source = Some(Source {
                id: music.get("id").unwrap().to_string(),
                kind: source::SourceKind::Netease
            });
            let url = None;
            let url_timeout = None;
            let cover = Some(music.get("al").unwrap().get("picUrl").unwrap().to_string());
            let title = music.get("name").unwrap().to_string();
            let album = Some(music.get("al").unwrap().get("name").unwrap().to_string());
            let artist = Some(music.get("ar").unwrap().as_array().unwrap().get(0).unwrap().get("name").unwrap().to_string());
            let year = None;
            let play_id = None;
            let requester = None;
            let duration = Duration::from_millis(music.get("dt").unwrap().as_u64().unwrap());

            let music_with_url = self.parse_music_url(Music {
                uuid,
                source,
                url,
                url_timeout,
                cover,
                title,
                album,
                artist,
                year,
                play_id,
                requester,
                duration,
            }).await?;
            music_list.push(music_with_url);
        };
        Ok(music_list)
    }

    async fn phone_num_login(&mut self) -> Result<()>{
        let res = reqwest::get(
            format!(
                "{}/login/cellphone?realIP=183.232.239.22&phone={}&password={}", 
                self.url.clone(),
                self.phone_num.clone(),
                self.password.clone()
            )).await?;
        match res.json::<Value>().await?.get("cookie") {
            Some(v) => {
                self.cookie = v.to_string();
            }
            None => return Err(anyhow::anyhow!("Login failed"))
        };
        Ok(())
    }
}

impl MusicApi for NeteaseApi {
    async fn login(&mut self) -> anyhow::Result<()> {
        info!("Loginning to NeteaseMusic Account");
        match fs::read_to_string("cookie.txt") {
            Ok(cookie) => {
                info!("You have logged in before, if you want to login again, please delete cookie.txt.");
                match self.check_cookie(Some(cookie.clone())).await {
                    Err(_) => {return Err(anyhow::anyhow!("Login Failed!"))},
                    Ok(_) => {
                        self.cookie = cookie;
                        info!("Login success!");
                    }
                } 
            },
            Err(_) => {
                info!("No Cookie.txt, try login with phone number");
                match self.phone_num_login().await {
                    Ok(_) => {
                        fs::write("cookie.txt", self.cookie.clone())?;
                        info!("Login success!");
                    },
                    Err(_) => return Err(anyhow::anyhow!("Login failed!"))
                }
            }
        }
        
        Ok(())
    }

    async fn get_music_by_id(&self, id: String) -> anyhow::Result<Music> {
        let res = reqwest::get(
            format!(
                "{}/song/detail?ids={}&cookie={}", 
                self.url.clone(),
                id,
                self.cookie.clone()
            )).await?;
        let res = res.json::<Value>().await?;
        if res.get("code").unwrap().as_u64().unwrap() != 200 {
            return Err(anyhow::anyhow!("Song not found"));
        }
        let music = self.parse_music(&res).await?;
        Ok(music.get(0).unwrap().clone())
    }

    async fn search_user(&self, name: String) -> anyhow::Result<Vec<User>> {
        let res = reqwest::get(
            format!(
                "{}/search?realIP=183.232.239.22&type=1002&keywords={}&cookie={}", 
                self.url.clone(),
                name,
                self.cookie.clone(),
            )).await?;
        let res = res.json::<Value>().await?;
        if res.get("code").unwrap().as_u64().unwrap() != 200 {
            return Err(anyhow::anyhow!("Search User failed!"));
        }
        let mut user_list = Vec::new();

        if let Some(result) = res.get("result").and_then(|v| v.as_object()) {
            if let Some(userprofiles) = result.get("userprofiles").and_then(|v| v.as_array()) {
                for userprofile in userprofiles.iter() {
                    let avatar = Some(userprofile.get("avatarUrl").unwrap().to_string());
                    let id = uuid::Uuid::new_v4();
                    let source = Some(Source {
                        id: userprofile.get("userId").unwrap().to_string(),
                        kind: source::SourceKind::Netease
                    });
                    let name = userprofile.get("nickname").unwrap().to_string();
                    user_list.push(User {
                        id,
                        avatar,
                        name,
                        source,
                    })
                }
            }
        }
        Ok(user_list)
    }
    async fn get_user_playlist(&self, id: String) -> anyhow::Result<Vec<source::PlayList>> {
        let res = reqwest::get(
        format!(
            "{}/user/playlist?realIP=183.232.239.22&uid={}&cookie={}", 
            self.url.clone(),
            id,
            self.cookie.clone(),
        )).await?;
        let res = res.json::<Value>().await?;
        if res.get("code").unwrap().as_u64().unwrap() != 200 {
            return Err(anyhow::anyhow!("Get user playlist failed!"));
        }
        let res = res.get("playlist").unwrap().as_array().unwrap();
        let mut playlist_list = Vec::new();
        for playlist in res.iter() {
            let source = Source {
                id: playlist.get("id").unwrap().to_string(),
                kind: source::SourceKind::Netease
            };
            let name = playlist.get("name").unwrap().to_string();
            playlist_list.push(PlayList {
                source,
                name
            });
        };
        Ok(playlist_list)
    }

    async fn get_music_by_playlist(&self, id: String, offset: u64) -> anyhow::Result<Vec<Music>> {
        let res = reqwest::get(
            format!(
                "{}/playlist/track/all?realIP=183.232.239.22&id={}&limit=10&offset={}&cookie={}", 
                self.url.clone(),
                id,
                offset,
                self.cookie.clone()
            )).await?;
        let res = res.json::<Value>().await?;
        if res.get("code").unwrap().as_u64().unwrap() != 200 {
            return Err(anyhow::anyhow!("Get music list failed!"));
        }
        let music = self.parse_music(&res).await?;
        Ok(music)
    }
}