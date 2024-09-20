use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Serialize)]
pub struct User {
    id: Uuid,
    name: String,
    avatar: String
}