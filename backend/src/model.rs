use std::{
    collections::HashMap,
    fmt::{
        self,
        Display,
    },
    str::FromStr,
    sync::Arc,
};

use chrono::{
    DateTime,
    Utc,
};
use rand::Rng;
use serde::{
    Deserialize,
    Deserializer,
    Serialize,
    de,
};
use serde_repr::{
    Deserialize_repr,
    Serialize_repr,
};
use socketioxide::socket::Sid;
use tokio::sync::Mutex;

// A character set for a base62 encoding.
const CHARSET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const ID_LENGTH: usize = 6;

// Function to generate a random ID of a given length.
fn generate_short_id() -> [char; ID_LENGTH] {
    let mut rng = rand::rng();
    let id = (0..ID_LENGTH)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect::<Vec<char>>();

    id.try_into()
        .unwrap_or_else(|_| panic!("Failed to convert Vec<char> to array"))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LobbyId(pub [char; ID_LENGTH]);

impl LobbyId {
    pub fn new() -> Self {
        Self(generate_short_id())
    }
}

impl FromStr for LobbyId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().count() != ID_LENGTH {
            return Err(format!(
                "Invalid length: expected {}, got {}",
                ID_LENGTH,
                s.len()
            ));
        }

        let chars: Vec<char> = s.chars().collect();
        let array: [char; ID_LENGTH] = chars
            .try_into()
            .map_err(|_| "Failed to convert to array".to_string())?;

        Ok(LobbyId(array))
    }
}

impl Display for LobbyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.iter().collect::<String>())
    }
}

impl<'de> Deserialize<'de> for LobbyId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

impl Serialize for LobbyId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s: String = String::from_iter(self.0);
        serializer.serialize_str(&s)
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct CardId(pub u8);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    id: CardId,
    description: String,
}

impl Card {
    pub fn new(description: String, idx: u8) -> Self {
        Self {
            description,
            id: CardId(idx),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LobbyManager {
    pub lobbies: Arc<Mutex<HashMap<LobbyId, Lobby>>>,
}

impl LobbyManager {
    pub fn new() -> Self {
        Self {
            lobbies: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn create_lobby(&self, lobby_id: LobbyId, host: Host, cards: [Card; 25]) {
        let mut lock = self.lobbies.lock().await;
        lock.insert(lobby_id, Lobby::new(host, cards));
    }

    pub async fn remove_lobby(&self, lobby_id: &LobbyId) -> Option<Lobby> {
        let mut lock = self.lobbies.lock().await;
        lock.remove(lobby_id)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("The lobby has already reached the last stage.")]
pub struct LastStateReached;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum LobbyState {
    /// The lobby is waiting for players to join - started by the host
    WaitingForPlayers,

    CraftingBoards,

    /// The lobby is currently in progress. No more players can join
    InProgress,

    /// The lobby has ended
    Completed,
}

impl LobbyState {
    pub fn next_stage(self) -> Option<Self> {
        let state = match self {
            LobbyState::WaitingForPlayers => LobbyState::CraftingBoards,
            LobbyState::CraftingBoards => LobbyState::InProgress,
            LobbyState::InProgress => LobbyState::Completed,
            LobbyState::Completed => return None,
        };

        Some(state)
    }
}

#[derive(Debug, Clone)]
pub struct Lobby {
    pub host: Host,
    pub available_cards: [Card; 25],
    pub players: Vec<Player>,
    pub correct_answers: Vec<CardId>,
    pub start_date: DateTime<Utc>,
    pub state: LobbyState,
    pub boards: HashMap<Sid, Board>,
}

impl Lobby {
    pub fn new(host: Host, available_cards: [Card; 25]) -> Self {
        Self {
            host,
            available_cards,
            players: Vec::new(),
            correct_answers: Vec::new(),
            start_date: Utc::now(),
            state: LobbyState::WaitingForPlayers,
            boards: HashMap::new(),
        }
    }

    pub fn is_host(&self, socket_id: Sid) -> bool {
        self.host.id == socket_id
    }

    pub fn advance_state(&mut self) -> Result<LobbyState, LastStateReached> {
        self.state = self.state.next_stage().ok_or(LastStateReached)?;
        Ok(self.state)
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
}

impl Player {
    pub fn new(id: Sid, name: String) -> Self {
        Self { id, name }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Board(pub [Card; 25]);
