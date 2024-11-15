use axum::async_trait;
use uuid::Uuid;
use crate::domain::models::Message;

#[async_trait]
pub trait Service {
    type Error;
    async fn join(&self, room_id: Uuid, user_id: Uuid) -> Result<(), Self::Error>;
    async fn leave(&self, room_id: Uuid, user_id: Uuid) -> Result<(), Self::Error>;
    async fn send(&self, room_id: Uuid, message: Message) -> Result<(), Self::Error>;
    async fn get_message(&self, room_id: Uuid, user_id: Uuid) -> Option<Vec<&Message>>;
}