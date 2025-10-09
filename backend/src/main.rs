// src/main.rs

pub mod model;
pub mod request;
pub mod response;
pub mod socket;

use axum::{
    Extension,
    Router,
};
use socketioxide::SocketIo;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::{
    model::GameManager,
    socket::event::on_connect,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up logging
    tracing_subscriber::fmt::init();

    let game_manager = GameManager::new();

    let (layer, io) = SocketIo::builder()
        .with_state(game_manager.clone())
        .build_layer();

    io.ns("/ws", on_connect);

    // Create the Axum application
    let app = Router::new()
        .with_state(game_manager)
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::very_permissive()) // Allow all origins for development
                .layer(layer),
        )
        .layer(Extension(io));

    info!("Starting server on http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
