use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub from: Uuid,
    pub id: Uuid,
    pub content: String,
}