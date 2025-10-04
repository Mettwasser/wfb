use std::{
    collections::HashMap,
    sync::Arc,
};

use serde::{
    Deserialize,
    Serialize,
};
use tokio::sync::Mutex;
use uuid::Uuid;

pub type LobbyId = Uuid;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct CardId(pub Uuid);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    id: Uuid,
    description: String,
}

impl Card {
    pub fn new(description: String) -> Self {
        Self {
            description,
            id: Uuid::new_v4(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameManager {
    lobbies: Arc<Mutex<HashMap<LobbyId, Game>>>,
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

#[derive(Debug, Clone)]
pub struct Game {
    pub host: Host,
    pub available_cards: [Card; 25],
    pub players: Vec<Player>,
    pub correct_answers: Vec<Uuid>,
}

impl Game {
    pub fn new(host: Host, available_cards: [Card; 25]) -> Self {
        Self {
            host,
            available_cards,
            players: Vec::new(),
            correct_answers: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Host {
    id: Uuid,
    name: String,
}

impl Host {
    pub fn new(id: Uuid, name: String) -> Self {
        Self { id, name }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    id: Uuid,
    name: String,
    board: Board,
}

impl Player {
    pub fn new(id: Uuid, name: String) -> Self {
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
