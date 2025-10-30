pub mod acks;
pub mod events;
pub mod request;

use std::str::FromStr;

use monostate::{
    MustBe,
    MustBeBool,
};
use serde::Serialize;
use socketioxide::{
    SocketIo,
    extract::{
        AckSender,
        Data,
        SocketRef,
        State,
    },
};
use tracing::{
    error,
    info,
    instrument,
};

use crate::{
    model::{
        Card,
        Host,
        LobbyId,
        LobbyManager,
        LobbyState,
        Player,
    },
    socket::{
        acks::{
            HostLobbyAck,
            JoinLobbyAck,
        },
        events::{
            ClientEvent,
            ServerEvent,
        },
        request::{
            BoardSubmitRequest,
            HostLobbyRequest,
            JoinLobbyRequest,
        },
    },
};

type True = MustBe!(true);
type False = MustBe!(false);

#[derive(Debug, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Acknowledgement<T = ()> {
    Success { success: True, data: T },
    Failure { success: False, message: String },
}

impl Acknowledgement {
    pub fn success<T>(data: T) -> Acknowledgement<T> {
        Acknowledgement::<T>::Success {
            success: MustBeBool,
            data,
        }
    }

    pub fn failure(message: impl Into<String>) -> Acknowledgement<()> {
        Acknowledgement::<()>::Failure {
            success: MustBeBool,
            message: message.into(),
        }
    }

    pub fn failure_t<T>(message: impl Into<String>) -> Acknowledgement<T> {
        Acknowledgement::<T>::Failure {
            success: MustBeBool,
            message: message.into(),
        }
    }
}

#[instrument(name = "socket.connect", skip(socket))]
pub async fn on_connect(socket: SocketRef) {
    info!("Socket connected: {}", socket.id);

    socket.on(ClientEvent::HostLobby, host_lobby);
    socket.on(ClientEvent::JoinLobby, join_lobby);
    socket.on(ClientEvent::TriggerNextStage, trigger_next_stage);
    socket.on(ClientEvent::SubmitBoard, submit_board);

    socket.on_disconnect(on_disconnect);
}

#[instrument(name = "socket.disconnect", skip(socket, io, manager))]
async fn on_disconnect(socket: SocketRef, io: SocketIo, State(manager): State<LobbyManager>) {
    info!("Socket disconnected: {}", socket.id);

    let mut lobbies = manager.lobbies.lock().await;

    let rooms = socket.rooms();

    for room in rooms {
        let Ok(lobby_id) = LobbyId::from_str(&room) else {
            continue;
        };

        if let Some(ref mut lobby) = lobbies.get_mut(&lobby_id) {
            info!("Socket {} was in lobby {}", socket.id, lobby_id);

            if let Some(idx) = lobby
                .players
                .iter()
                .position(|player| player.id == socket.id)
            {
                let player = lobby.players.remove(idx);
                io.to(room)
                    .emit(ServerEvent::UserLeft, &player.name)
                    .await
                    .ok();
            } else if lobby.host.id == socket.id {
                lobbies.remove(&lobby_id);
                info!(%lobby_id, lobby_count = lobbies.len(), "deleted lobby");

                io.within(room.clone())
                    .leave(room)
                    .await
                    .expect("Failed to disconnect");
            }
        }
    }
}

#[instrument(name = "lobby.host", skip(socket, manager, ack))]
async fn host_lobby(
    socket: SocketRef,
    Data(request): Data<HostLobbyRequest>,
    State(manager): State<LobbyManager>,
    ack: AckSender,
) {
    info!(
        ?request,
        "Socket {} is attempting to host a lobby ", socket.id
    );

    let lobby_id = LobbyId::new();

    let host = Host::new(socket.id, request.host_name);

    let cards: [Card; 25] = request
        .cards
        .iter()
        .enumerate()
        .map(|(idx, card)| Card::new(card.clone(), idx as u8))
        .collect::<Vec<Card>>()
        .try_into()
        .expect("Failed to convert cards to array");

    manager
        .create_lobby(lobby_id, host.clone(), cards.clone())
        .await;

    socket.join(lobby_id.to_string());

    match ack.send(&Acknowledgement::success(HostLobbyAck {
        lobby_id: lobby_id.to_string(),
        cards,
    })) {
        Ok(_) => info!("Successfully hosted lobby {}", lobby_id),
        Err(err) => {
            error!("Failed to host lobby {}: {}", lobby_id, err);
            manager.remove_lobby(&lobby_id).await;
        }
    }
}

#[instrument(name = "lobby.join", skip(socket, io, manager, ack))]
async fn join_lobby(
    socket: SocketRef,
    io: SocketIo,
    Data(request): Data<JoinLobbyRequest>,
    State(manager): State<LobbyManager>,
    ack: AckSender,
) {
    info!(
        "Socket {} is attempting to join lobby {}",
        socket.id, request.lobby_id
    );

    let mut lobbies = manager.lobbies.lock().await;

    // Check if the lobby exists
    let Some(lobby) = lobbies.get_mut(&request.lobby_id) else {
        error!("Lobby {} does not exist", request.lobby_id);
        let _ = ack.send(&Acknowledgement::failure("Lobby does not exist"));
        return;
    };

    // Check if the lobby is accepting new players
    if lobby.state != LobbyState::WaitingForPlayers {
        error!("Lobby {} is not accepting new players", request.lobby_id);
        let _ = ack.send(&Acknowledgement::failure(
            "Lobby is not accepting new players",
        ));
        return;
    }

    // Check if the player name is already taken
    if lobby
        .players
        .iter()
        .find(|player| player.name == request.player_name)
        .is_some()
        || lobby.host.name == request.player_name
    {
        error!(
            "Player name {} is already taken in lobby {}",
            request.player_name, request.lobby_id
        );
        let _ = ack.send(&Acknowledgement::failure("Player name is already taken"));
        return;
    }

    lobby
        .players
        .push(Player::new(socket.id, request.player_name.clone()));

    let players: Vec<String> = lobby
        .players
        .iter()
        .map(|player| player.name.clone())
        .collect();

    match ack.send(&Acknowledgement::success(JoinLobbyAck {
        players,
        host: lobby.host.name.clone(),
        cards: lobby.available_cards.clone(),
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
        .within(request.lobby_id.to_string())
        .emit(ServerEvent::UserJoined, &request.player_name)
        .await
    {
        error!("Failed to emit user joined event: {}", err);
    }
}

#[instrument(name = "lobby.next_stage", skip(socket, io, manager))]
pub async fn trigger_next_stage(
    socket: SocketRef,
    io: SocketIo,
    Data(lobby_id): Data<LobbyId>,
    State(manager): State<LobbyManager>,
) {
    let mut lobbies = manager.lobbies.lock().await;

    let Some(lobby) = lobbies.get_mut(&lobby_id) else {
        return;
    };

    if !lobby.is_host(socket.id) {
        return;
    }

    let Ok(state) = lobby.advance_state() else {
        return;
    };

    io.to(lobby_id.to_string())
        .emit(ServerEvent::NextStage, &state)
        .await
        .ok();
}

#[instrument(name = "lobby.board_submitted", skip(socket, io, manager, ack))]
pub async fn submit_board(
    socket: SocketRef,
    io: SocketIo,
    Data(req): Data<BoardSubmitRequest>,
    State(manager): State<LobbyManager>,
    ack: AckSender,
) {
    let mut lobbies = manager.lobbies.lock().await;

    let Some(lobby) = lobbies.get_mut(&req.lobby_id) else {
        return;
    };

    lobby.boards.insert(socket.id, req.cards);

    let player_name = lobby
        .players
        .iter()
        .find(|p| p.id == socket.id)
        .unwrap()
        .name
        .as_str();

    match ack.send(&Acknowledgement::success(())) {
        Ok(_) => {
            io.within(req.lobby_id.to_string())
                .emit(ServerEvent::BoardSubmitted, player_name)
                .await
                .ok();
        }
        Err(err) => {
            error!("Failed to send submit board ack: {}", err);
        }
    };
}
