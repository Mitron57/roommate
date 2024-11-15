use axum::async_trait;

#[async_trait]
pub trait Cache<K, I> {
    async fn insert(&self, key: K, item: I);
    async fn erase(&self, key: K);
    async fn get(&self, key: &K) -> Option<&I>;
}