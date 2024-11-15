use crate::domain::interfaces;
use crate::domain::interfaces::Cache;
use crate::domain::models::Room;
use axum::async_trait;
use uuid::Uuid;

pub struct RoomRepository<C> {
    rooms: C,
}

impl<C: Cache<Uuid, Room> + Send + Sync> RoomRepository<C> {
    pub fn new(rooms: C) -> RoomRepository<C> {
        RoomRepository { rooms }
    }
}

#[async_trait]
impl<C> interfaces::Repository<Uuid, Room> for RoomRepository<C>
where
    C: Cache<Uuid, Room> + Send + Sync,
{
    async fn insert(&self, item: Room) {
        self.rooms.insert(item.id, item).await;
    }

    async fn erase(&self, key: Uuid) {
        self.rooms.erase(key).await;
    }

    async fn get(&self, key: &Uuid) -> Option<&Room> {
        self.rooms.get(key).await
    }
}
