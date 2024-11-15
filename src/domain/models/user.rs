use uuid::Uuid;

#[derive(Eq, Hash, PartialEq)]
pub struct User {
    pub id: Uuid,
}