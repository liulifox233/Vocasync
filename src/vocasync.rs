use std::sync::Arc;
use anyhow::Result;
use std::ops::Deref;
use crate::api::netease::NeteaseApi;
use crate::music::Music;
use crate::source::MusicApi;
use crate::{
    config::Config,
    room::Room
};
use sqlx::postgres;


pub struct Inner{
    pub config: Config,
    pub pg_pool: postgres::PgPool,
    pub room: Room,
    pub neteaseapi: NeteaseApi
}

#[derive(Clone)]
pub struct Vocasync(Arc<Inner>);
impl Vocasync{
    pub async fn new(config: Config) -> Result<Self> {
        config.check().await?;
        let pg_pool = postgres::PgPool::connect_lazy(&config.database_url)?;

        let room = Room::new().await?.into();

        let mut neteaseapi = NeteaseApi {
            url: config.neteaseapi.url.clone(),
            phone_num: config.neteaseapi.phone_num.clone(),
            password: config.neteaseapi.password.clone(),
            cookie: "".to_string()
        };
        neteaseapi.login().await?;
        
        let inner = Arc::new(Inner {
            config,
            pg_pool,
            room,
            neteaseapi,
        });
        let res = Self(inner);
        Ok(res)
    }

    pub async fn save_music(&self, music: Music) -> Result<()> {
        //TODO
        Ok(())
    }
}

impl Deref for Vocasync {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


