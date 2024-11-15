use axum::async_trait;

#[async_trait]
pub trait Repository<K, I> {
    async fn insert(&self, item: I);
    async fn erase(&self, key: K);
    async fn get(&self, key: &K) -> Option<&I>;
}