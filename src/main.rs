use std::error::Error;
use crate::application::{AppState, Chat};
use crate::infrastructure::{Cache, RoomRepository};
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;
use log::info;

mod application;
mod domain;
mod handlers;
mod infrastructure;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    if dotenvy::dotenv().is_err() {
        println!("Failed to load .env file, using manually specified env variables");
    }
    env_logger::init();
    let cache = Cache::new();
    let rooms = Arc::new(RoomRepository::new(cache));
    let chat = Arc::new(Chat::new(rooms));
    let state = Arc::new(AppState { chat });

    let hostaddr = std::env::var("HOSTADDR")?;

    let app = Router::new()
        .route("/join", post(handlers::join))
        .route("/leave", post(handlers::leave))
        .route("/send", post(handlers::send))
        .route("/messages", get(handlers::messages)).with_state(state);

    let listener = tokio::net::TcpListener::bind(&hostaddr).await?;
    info!("Listening on {}", hostaddr);
    axum::serve(listener, app).await?;
    Ok(())
}
