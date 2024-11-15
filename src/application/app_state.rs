use std::error;
use std::sync::Arc;
use crate::domain::interfaces::Service;

type Error = Box<dyn error::Error + Send + Sync>;
pub struct AppState {
    pub chat: Arc<dyn Service<Error=Error> + Send + Sync>,
}