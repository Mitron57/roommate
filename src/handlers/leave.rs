use crate::application::AppState;
use crate::handlers::body::UserAndRoom;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use std::io;
use std::sync::Arc;

pub async fn leave(State(state): State<Arc<AppState>>, Json(body): Json<UserAndRoom>) -> impl IntoResponse {
    if let Err(err) = state.chat.leave(body.room_id, body.user_id).await {
        if let Some(err) = err.downcast_ref::<io::Error>() {
            if err.kind() == io::ErrorKind::NotFound {
                return (StatusCode::BAD_REQUEST, "invalid room id");
            }
        }
    }
    (StatusCode::OK, "exit success")
}
