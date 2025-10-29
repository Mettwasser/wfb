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
    model::LobbyManager,
    socket::on_connect,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up logging
    tracing_subscriber::fmt::init();

    let lobby_manager = LobbyManager::new();

    let (layer, io) = SocketIo::builder()
        .with_state(lobby_manager.clone())
        .build_layer();

    io.ns("/", on_connect);

    // Create the Axum application
    let app = Router::new()
        .with_state(lobby_manager)
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
