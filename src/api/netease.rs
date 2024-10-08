use crate::{music::Music, source::{MusicApi, PlayList, Source}, user::User};
use anyhow::Result;
use reqwest::{Client, Url};
use serde_json::Value;
use tracing::info;
use uuid::Uuid;
use std::{fs, sync::Arc, time::Duration};
use std::time::SystemTime;

use crate::source;


pub struct NeteaseApi {
    pub url: String,
    pub phone_num: String,
    pub password: String,
    pub cookie: String,
    pub client: Arc<Client>
}

impl NeteaseApi {
    pub async fn init(
        url: String,
        phone_num: String,
        password: String
    ) -> Result<Self> {
        let client = Arc::new(Client::new());
        Ok(NeteaseApi {
            url,
            phone_num,
            password,
            cookie: String::new(),
            client
        })
    }

    async fn check_cookie(&self, cookie: Option<String>) -> Result<()> {
        if cookie.is_none() {return Err(anyhow::anyhow!("Please check your cookie.txt"));}
        let url = format!(
            "{}/user/account?realIP=183.232.239.22&timestamp={}&cookie={}",
            self.url,
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs(),
            cookie.unwrap()
        );
        let res = self.client.get(&url).send().await?.json::<Value>().await?;
        if res.get("profile").map_or(true, |profile| profile.is_null()) {
            return Err(anyhow::anyhow!("Please check your cookie.txt"));
        }
        Ok(())
    }
    
    async fn parse_music_url(&self, mut music: Music) -> Result<Music> {
        let url = format!(
            "{}/song/url?realIP=183.232.239.22&id={}&cookie={}", 
            self.url.clone(),
            music.source.clone().unwrap().id,
            self.cookie.clone()
        );
        let res = self.client.get(&url).send().await?.json::<Value>().await?;
        if res.get("code").unwrap().as_u64().unwrap() != 200 {
            return Err(anyhow::anyhow!("Get song failed!"));
        }else {
            music.url = match res.get("data").unwrap().as_array().unwrap().get(0).unwrap().get("url") {
                Some(r) => Some(r.as_str().unwrap().to_string()),
                None => return Err(anyhow::anyhow!("Get song failed!"))
            };
        }
        Ok(music)
    }

    async fn parse_music(&self, res: &Value) -> Result<Vec<Music>> {
        let songs = res.get("songs")
            .and_then(|s| s.as_array())
            .ok_or_else(|| anyhow::anyhow!("Invalid response format"))?;

        let mut music_list = Vec::new();

        for song in songs {
            let music = Music {
                uuid: Uuid::new_v4(),
                source: Some(Source {
                    id: song.get("id")
                        .and_then(|id| id.as_u64())
                        .ok_or_else(|| anyhow::anyhow!("Missing song id"))?
                        .to_string(),
                    kind: source::SourceKind::Netease,
                }),
                url: None,
                url_timeout: None,
                cover: song.get("al")
                    .and_then(|al| al.get("picUrl"))
                    .and_then(|url| url.as_str())
                    .map(|url| url.to_string()),
                title: song.get("name")
                    .and_then(|name| name.as_str())
                    .ok_or_else(|| anyhow::anyhow!("Missing song name"))?
                    .to_string(),
                album: song.get("al")
                    .and_then(|al| al.get("name"))
                    .and_then(|name| name.as_str())
                    .map(|name| name.to_string()),
                artist: song.get("ar")
                    .and_then(|ar| ar.as_array())
                    .and_then(|ar| ar.get(0))
                    .and_then(|artist| artist.get("name"))
                    .and_then(|name| name.as_str())
                    .map(|name| name.to_string()),
                year: None,
                play_id: None,
                requester: None,
                duration: Duration::from_millis(
                    song.get("dt")
                        .and_then(|dt| dt.as_u64())
                        .ok_or_else(|| anyhow::anyhow!("Missing song duration"))?
                ),
            };

            let music_with_url = self.parse_music_url(music).await?;
            music_list.push(music_with_url);
        }

        Ok(music_list)
    }

    async fn phone_num_login(&mut self) -> Result<()>{
        let url = format!(
            "{}/login/cellphone?realIP=183.232.239.22&phone={}&password={}", 
            self.url.clone(),
            self.phone_num.clone(),
            self.password.clone()
        );
        let res = self.client.get(&url).send().await?.json::<Value>().await?;
        match res.get("cookie") {
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
        let url = format!(
            "{}/song/detail?ids={}&cookie={}", 
            self.url.clone(),
            id,
            self.cookie.clone()
        );
        let res = self.client.get(&url).send().await?.json::<Value>().await?;
        if res.get("code").unwrap().as_u64().unwrap() != 200 || res.get("songs").unwrap().as_array().unwrap().len() == 0 {
            return Err(anyhow::anyhow!("Song not found"));
        }
        let music = self.parse_music(&res).await?;
        Ok(music.get(0).unwrap().clone())
    }

    async fn search_user(&self, name: String) -> anyhow::Result<Vec<User>> {
        let url = format!(
            "{}/search?realIP=183.232.239.22&type=1002&keywords={}&cookie={}", 
            self.url.clone(),
            name,
            self.cookie.clone(),
        );
        let res = self.client.get(&url).send().await?.json::<Value>().await?;
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
        let url = format!(
            "{}/user/playlist?realIP=183.232.239.22&uid={}&cookie={}", 
            self.url.clone(),
            id,
            self.cookie.clone(),
        );
        let res = self.client.get(&url).send().await?.json::<Value>().await?;
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
        let url = format!(
            "{}/playlist/track/all?realIP=183.232.239.22&id={}&limit=10&offset={}&cookie={}", 
            self.url.clone(),
            id,
            offset,
            self.cookie.clone()
        );
        let res = self.client.get(&url).send().await?.json::<Value>().await?;
        if res.get("code").unwrap().as_u64().unwrap() != 200 {
            return Err(anyhow::anyhow!("Get music list failed!"));
        }
        let music = self.parse_music(&res).await?;
        Ok(music)
    }
}