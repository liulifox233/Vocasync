use std::{sync::Arc, thread::sleep, time};
use deadpool::managed::Pool;
use deadpool_postgres;
use anyhow::Result;
use tokio_postgres::NoTls;
use std::ops::Deref;
use crate::{
    config::Config,
    music::{CurrentMusic, SerializeCurrentMusic}, 
    room::Room
};

pub struct Inner{
    pub config: Config,
    pub pg_pool: Pool<deadpool_postgres::Manager>,
    pub room: Room
}

#[derive(Clone)]
pub struct Vocasync(Arc<Inner>);
impl Vocasync{
    pub async fn new(config: Config) -> Result<Self> {
        config.check().await?;

        let pg_pool = config
            .database
            .create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
        let room = Room::new().await?;

        let inner = Arc::new(Inner {
            config,
            pg_pool,
            room
        });
        let res = Self(inner);
        Ok(res)
    }

}

impl Deref for Vocasync {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

