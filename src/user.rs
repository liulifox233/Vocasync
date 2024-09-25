use serde::{de::value::UsizeDeserializer, Deserialize, Serialize};
use tower_sessions::Session;
use uuid::Uuid;
use anyhow::Result;

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    id: Uuid,
    name: String,
    avatar: Option<String>
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
                    name,
                    avatar
                }
            }
        };
        Ok(user)
    }
}