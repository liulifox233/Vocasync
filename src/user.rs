use serde::{de::value::UsizeDeserializer, Deserialize, Serialize};
use tower_sessions::Session;
use uuid::Uuid;
use anyhow::Result;

use crate::source::Source;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub source: Option<Source>,
    pub name: String,
    pub avatar: Option<String>
}

impl User {
    pub async fn create(session: Session) -> Result<User> {
        let user = match session.get::<User>("id").await? {
            Some(u) => u,
            None => {
                let id = Uuid::new_v4();
                let name = String::from(id);
                let avatar = None;
                User {
                    id,
                    source: None,
                    name,
                    avatar
                }
            }
        };
        Ok(user)
    }
}