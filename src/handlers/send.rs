use crate::application::AppState;
use crate::handlers::body::UserMessage;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use std::io;
use std::sync::Arc;

pub async fn send(
    State(state): State<Arc<AppState>>,
    Json(body): Json<UserMessage>,
) -> impl IntoResponse {
    if let Err(err) = state.chat.send(body.room_id, body.message).await {
        if let Some(err) = err.downcast_ref::<io::Error>() {
            if err.kind() == io::ErrorKind::NotFound {
                return (StatusCode::BAD_REQUEST, "User with this id not in room or incorrect room_id");
            }
        }
    }
    (StatusCode::OK, "")
}
