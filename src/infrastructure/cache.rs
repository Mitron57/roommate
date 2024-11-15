use crate::domain::interfaces;
use axum::async_trait;
use dashmap::DashMap;
use std::hash::Hash;

pub struct Cache<Id, I> {
    items: DashMap<Id, I>,
}

impl<Id: Eq + Hash, I> Cache<Id, I> {
    pub fn new() -> Self {
        Self {
            items: DashMap::new(),
        }
    }
}
#[async_trait]
impl<Id: Eq + Hash, I> interfaces::Cache<Id, I> for Cache<Id, I>
where
    Id: Send + Sync,
    I: Send + Sync,
{
    async fn insert(&self, key: Id, item: I) {
        self.items.insert(key, item);
    }

    async fn erase(&self, key: Id) {
        self.items.remove(&key);
    }

    async fn get(&self, id: &Id) -> Option<&I> {
        if let Some(item) = self.items.get(&id) {
            Some(unsafe { &*(item.value() as *const I) })
        } else {
            None
        }
    }
}
