use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::models::Message;

#[derive(Serialize, Deserialize)]
pub struct UserAndRoom {
    pub user_id: Uuid,
    pub room_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct UserMessage {
    pub room_id: Uuid,
    pub message: Message
}