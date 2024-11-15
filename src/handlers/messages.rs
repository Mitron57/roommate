use crate::application::AppState;
use crate::handlers::body::UserAndRoom;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};
use std::sync::Arc;

pub async fn messages(
    State(state): State<Arc<AppState>>,
    Json(body): Json<UserAndRoom>,
) -> (StatusCode, Json<Value>) {
    if let Some(messages) = state.chat.get_message(body.room_id, body.user_id).await {
        return (StatusCode::OK, Json(json!(messages)));
    }
    (StatusCode::NOT_FOUND, Json(json!({})))
}
