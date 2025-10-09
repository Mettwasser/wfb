use std::{
    borrow::Cow,
    str::FromStr,
};

use serde::Serialize;
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
        GameState,
        Host,
        LobbyId,
    },
    socket::{
        acks::JoinLobbyAck,
        request::{
            HostLobbyRequest,
            JoinLobbyRequest,
        },
    },
};

#[derive(Debug, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Acknowledgement<T = ()> {
    Success { success: bool, data: T },
    Failure { success: bool, message: String },
}

impl<T> Acknowledgement<T> {
    pub fn success(data: T) -> Self {
        Self::Success {
            success: true,
            data,
        }
    }
}

impl Acknowledgement {
    pub fn failure(message: impl Into<String>) -> Self {
        Self::Failure {
            success: false,
            message: message.into(),
        }
    }
}

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
    UserJoined,
}

impl AsRef<str> for ServerEvent {
    fn as_ref(&self) -> &str {
        match self {
            ServerEvent::UserJoined => "userJoined",
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
        let Ok(lobby_id) = LobbyId::from_str(&room) else {
            continue;
        };

        if let Some(game) = lobbies.get(&lobby_id)
            && game.host.id == socket.id
        {
            info!("Socket {} was in room {}", socket.id, lobby_id);
            lobbies.remove(&lobby_id);
            io.of("/ws")
                .unwrap()
                .within(room.clone())
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

    let lobby_id = LobbyId::new();

    let host = Host::new(socket.id, request.host_name);

    let cards = request.cards.map(Card::new);

    manager.create_lobby(lobby_id, host.clone(), cards).await;

    socket.join(lobby_id.to_string());

    match ack.send(&lobby_id) {
        Ok(_) => info!("Successfully hosted lobby {}", lobby_id),
        Err(err) => {
            error!("Failed to host lobby {}: {}", lobby_id, err);
            manager.remove_lobby(&lobby_id).await;
        }
    }
}

#[instrument(name = "lobby.join", skip(ack))]
async fn join_lobby(
    io: SocketIo,
    socket: SocketRef,
    Data(request): Data<JoinLobbyRequest>,
    State(state): State<GameManager>,
    ack: AckSender,
) {
    info!(
        "Socket {} is attempting to join lobby {}",
        socket.id, request.lobby_id
    );

    let lobby_id = match LobbyId::from_str(&request.lobby_id) {
        Ok(lobby_id) => lobby_id,
        Err(err) => {
            error!("Invalid lobby ID {}: {}", request.lobby_id, err);
            let _ = ack.send(&Acknowledgement::failure("Invalid lobby ID"));
            return;
        }
    };

    // Check if the lobby exists
    let Some(game) = state.get_lobby(&lobby_id).await else {
        error!("Lobby {} does not exist", lobby_id);
        let _ = ack.send(&Acknowledgement::failure("Lobby does not exist"));
        return;
    };

    // Check if the lobby is accepting new players
    if game.state != GameState::WaitingForPlayers {
        error!("Lobby {} is not accepting new players", request.lobby_id);
        let _ = ack.send(&Acknowledgement::failure(
            "Lobby is not accepting new players",
        ));
        return;
    }

    // Check if the player name is already taken
    if game
        .players
        .iter()
        .find(|player| player.name == request.player_name)
        .is_some()
        || game.host.name == request.player_name
    {
        error!(
            "Player name {} is already taken in lobby {}",
            request.player_name, request.lobby_id
        );
        let _ = ack.send(&Acknowledgement::failure("Player name is already taken"));
        return;
    }

    let players: Vec<String> = game
        .players
        .iter()
        .map(|player| player.name.clone())
        .collect();

    match ack.send(&Acknowledgement::success(JoinLobbyAck {
        players,
        host: game.host.name.clone(),
    })) {
        Ok(_) => {
            socket.join(request.lobby_id.to_string());
        }
        Err(err) => {
            error!("Failed to send join lobby ack: {}", err);
            return;
        }
    };

    if let Err(err) = io
        .of("/ws")
        .unwrap()
        .within(request.lobby_id.to_string())
        .emit(ServerEvent::UserJoined, &request.player_name)
        .await
    {
        error!("Failed to emit user joined event: {}", err);
    }
}
