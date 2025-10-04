// src/main.rs

pub mod model;
pub mod request;
pub mod response;
pub mod socket_event;

use axum::{
    Extension,
    Json,
    Router,
    debug_handler,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
};
use axum_extra::extract::{
    CookieJar,
    cookie::Cookie,
};
use socketioxide::SocketIo;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;
use uuid::Uuid;

use crate::{
    model::{
        Card,
        GameManager,
        Host,
    },
    request::CreateLobbyRequest,
};

#[debug_handler]
async fn create_lobby(
    jar: CookieJar,
    manager: State<GameManager>,
    Json(req): Json<CreateLobbyRequest>,
) -> impl IntoResponse {
    let lobby_id = Uuid::new_v4();
    let host_id = Uuid::new_v4();

    let host = Host::new(host_id, req.host_name);

    let cards = req.cards.map(Card::new);

    manager.create_lobby(lobby_id, host.clone(), cards).await;

    (
        StatusCode::CREATED,
        jar.add(Cookie::new("lobbyId", lobby_id.to_string()))
            .add(Cookie::new("playerId", host_id.to_string())),
        Json(host),
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up logging
    tracing_subscriber::fmt::init();

    let game_manager = GameManager::new();

    let (layer, io) = SocketIo::new_layer();

    // io.ns("/ws", |socket: SocketRef| {});

    // Create the Axum application
    let app = Router::new()
        .nest(
            "/api",
            Router::new().nest("/lobby", Router::new().route("/", post(create_lobby))),
        )
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
