use std::borrow::Cow;

use socketioxide::{
    SocketIo,
    adapter::LocalAdapter,
    extract::{
        AckSender,
        Data,
        SocketRef,
        State,
    },
    handler::{
        MessageHandler,
        Value,
    },
};
use tracing::{
    error,
    info,
    instrument,
};
use uuid::Uuid;

use crate::{
    model::{
        Card,
        GameManager,
        Host,
    },
    socket::request::{
        HostLobbyRequest,
        JoinLobbyRequest,
    },
};

#[derive(Debug)]
struct AuthError;

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AuthError")
    }
}
impl std::error::Error for AuthError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ClientEvent {
    HostLobby,
    JoinLobby,
}

impl From<ClientEvent> for Cow<'static, str> {
    fn from(value: ClientEvent) -> Self {
        match value {
            ClientEvent::HostLobby => Cow::Borrowed("hostLobby"),
            ClientEvent::JoinLobby => Cow::Borrowed("joinLobby"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ServerEvent {
    NameAlreadyTaken,
}

impl AsRef<str> for ServerEvent {
    fn as_ref(&self) -> &str {
        match self {
            ServerEvent::NameAlreadyTaken => "nameAlreadyTaken",
        }
    }
}

#[instrument(name = "socket.connect")]
pub fn on_connect(socket: SocketRef) {
    info!("Socket connected: {}", socket.id);

    socket.on(ClientEvent::HostLobby, host_lobby);
    socket.on(ClientEvent::JoinLobby, join_lobby);

    socket.on_disconnect(on_disconnect);
}

#[instrument(name = "socket.disconnect")]
async fn on_disconnect(socket: SocketRef, io: SocketIo, State(manager): State<GameManager>) {
    info!("Socket disconnected: {}", socket.id);

    let mut lobbies = manager.lobbies.lock().await;

    let rooms = socket.rooms();

    if rooms.len() > 1 {
        info!(?rooms, "Socket {} was in more than one room", socket.id);
    }

    for room in rooms {
        if let Ok(lobby_id) = Uuid::parse_str(&room)
            && let Some(game) = lobbies.get(&lobby_id)
            && game.host.id == socket.id
        {
            info!("Socket {} was in room {}", socket.id, room);
            lobbies.remove(&lobby_id);
            io.of("/ws")
                .unwrap()
                // .within(room)
                .leave(room)
                .await
                .expect("Failed to disconnect");
        }
    }
}

#[instrument(name = "lobby.host", skip(ack))]
async fn host_lobby(
    socket: SocketRef,
    Data(request): Data<HostLobbyRequest>,
    State(manager): State<GameManager>,
    ack: AckSender,
) {
    info!(
        ?request,
        "Socket {} is attempting to host a lobby ", socket.id
    );

    let lobby_id = Uuid::new_v4();

    let host = Host::new(socket.id, request.host_name);

    let cards = request.cards.map(Card::new);

    manager.create_lobby(lobby_id, host.clone(), cards).await;

    socket.join(lobby_id.to_string());

    match ack.send(&host) {
        Ok(_) => info!("Successfully hosted lobby {}", lobby_id),
        Err(err) => {
            error!("Failed to host lobby {}: {}", lobby_id, err);
            manager.remove_lobby(&lobby_id).await;
        }
    }
}

#[instrument(name = "lobby.join", skip(ack))]
async fn join_lobby(
    socket: SocketRef,
    Data(request): Data<JoinLobbyRequest>,
    State(state): State<GameManager>,
    ack: AckSender,
) {
    info!(
        "Socket {} is attempting to join lobby {}",
        socket.id, request.lobby_id
    );

    todo!()
}
