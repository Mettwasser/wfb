use std::{
    collections::HashMap,
    sync::Arc,
};

use chrono::{
    DateTime,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};
use socketioxide::socket::Sid;
use tokio::sync::Mutex;
use uuid::Uuid;

pub type LobbyId = Uuid;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct CardId(pub Uuid);

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct PlayerId(pub Uuid);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    id: CardId,
    description: String,
}

impl Card {
    pub fn new(description: String) -> Self {
        Self {
            description,
            id: CardId(Uuid::new_v4()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameManager {
    pub lobbies: Arc<Mutex<HashMap<LobbyId, Game>>>,
}

impl GameManager {
    pub fn new() -> Self {
        Self {
            lobbies: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn create_lobby(&self, lobby_id: LobbyId, host: Host, cards: [Card; 25]) {
        let mut lock = self.lobbies.lock().await;
        lock.insert(lobby_id, Game::new(host, cards));
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    /// The game is waiting for players to join - started by the host
    WaitingForPlayers,

    /// The game is currently in progress. No more players can join
    InProgress,

    /// The game has ended
    Completed { winners: Vec<PlayerId> },
}

#[derive(Debug, Clone)]
pub struct Game {
    pub host: Host,
    pub available_cards: [Card; 25],
    pub players: Vec<Player>,
    pub correct_answers: Vec<CardId>,
    pub start_date: DateTime<Utc>,
    pub state: GameState,
}

impl Game {
    pub fn new(host: Host, available_cards: [Card; 25]) -> Self {
        Self {
            host,
            available_cards,
            players: Vec::new(),
            correct_answers: Vec::new(),
            start_date: Utc::now(),
            state: GameState::WaitingForPlayers,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Host {
    pub name: String,
    pub id: Sid,
}

impl Host {
    pub fn new(id: Sid, name: String) -> Self {
        Self { id, name }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub id: Sid,
    pub name: String,
    pub board: Board,
}

impl Player {
    pub fn new(id: Sid, name: String) -> Self {
        Self {
            id,
            name,
            board: Board([
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None,
            ]),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Board(pub [Option<Card>; 25]);
