use std::sync::atomic::AtomicUsize;
use dashmap::{DashMap, DashSet};
use uuid::Uuid;
use crate::domain::models::message::Message;
use crate::domain::models::user::User;

pub struct Room {
    pub id: Uuid,
    pub users_count: AtomicUsize,
    pub users: DashSet<User>,
    pub messages: DashMap<Uuid, Message>
}