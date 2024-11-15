use crate::domain::interfaces;
use crate::domain::interfaces::Repository;
use crate::domain::models::{Message, Room, User};
use axum::async_trait;
use std::error::Error;
use std::io;
use std::io::ErrorKind;
use std::ops::Deref;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use uuid::Uuid;

pub struct Chat {
    rooms: Arc<dyn Repository<Uuid, Room> + Send + Sync>,
}

impl Chat {
    pub fn new(rooms: Arc<dyn Repository<Uuid, Room> + Send + Sync>) -> Chat {
        Chat { rooms }
    }
}

#[async_trait]
impl interfaces::Service for Chat {
    type Error = Box<dyn Error + Send + Sync>;

    async fn join(&self, room_id: Uuid, user_id: Uuid) -> Result<(), Self::Error> {
        match self.rooms.get(&room_id).await {
            Some(room) => {
                room.users.insert(User { id: user_id });
                Ok(())
            }
            None => Err(io::Error::from(ErrorKind::NotFound).into()),
        }
    }

    async fn leave(&self, room_id: Uuid, user_id: Uuid) -> Result<(), Self::Error> {
        match self.rooms.get(&room_id).await {
            Some(room) => {
                if room.users.remove(&User { id: user_id }).is_some() {
                    room.users_count.fetch_sub(1, Ordering::AcqRel);
                }
                Ok(())
            }
            None => Err(io::Error::from(ErrorKind::NotFound).into()),
        }
    }

    async fn send(&self, room_id: Uuid, message: Message) -> Result<(), Self::Error> {
        match self.rooms.get(&room_id).await {
            Some(room) => {
                if room.users.contains(&User { id: message.from }) {
                    room.messages.insert(message.id, message);
                    return Ok(());
                }
                Err(io::Error::from(ErrorKind::NotFound).into())
            }
            None => Err(io::Error::from(ErrorKind::NotFound).into()),
        }
    }

    async fn get_message(&self, room_id: Uuid, user_id: Uuid) -> Option<Vec<&Message>> {
        match self.rooms.get(&room_id).await {
            Some(room) => {
                if room.users.contains(&User { id: user_id }) {
                    Some(
                        room.messages
                            .iter()
                            .map(|msg| unsafe { &*(msg.deref() as *const _) })
                            .collect(),
                    )
                } else {
                    None
                }
            }
            None => None,
        }
    }
}
