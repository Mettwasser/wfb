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
    Serialize,
};
use socketioxide::socket::Sid;
use tokio::sync::Mutex;
use uuid::Uuid;

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

mod char_vec_as_string {
    use serde::{
        Deserializer,
        Serializer,
        de,
    };

    use super::*;

    pub fn serialize<S>(chars: &[char; ID_LENGTH], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Collect the Vec<char> into a String and serialize the string
        let s: String = String::from_iter(chars);
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[char; ID_LENGTH], D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        // Ensure the string has the correct length for our fixed-size array
        if s.chars().count() != ID_LENGTH {
            let expected = ID_LENGTH.to_string();
            return Err(de::Error::invalid_length(
                s.chars().count(),
                &expected.as_str(),
            ));
        }

        let mut array = ['\0'; ID_LENGTH]; // Initialize with a placeholder
        for (i, c) in s.chars().enumerate() {
            array[i] = c;
        }
        Ok(array)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LobbyId(#[serde(with = "char_vec_as_string")] pub [char; ID_LENGTH]);

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

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct CardId(pub Uuid);

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

    pub async fn get_lobby(&self, lobby_id: &LobbyId) -> Option<Game> {
        let lock = self.lobbies.lock().await;
        lock.get(lobby_id).cloned()
    }

    pub async fn remove_lobby(&self, lobby_id: &LobbyId) -> Option<Game> {
        let mut lock = self.lobbies.lock().await;
        lock.remove(lobby_id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    /// The game is waiting for players to join - started by the host
    WaitingForPlayers,

    CraftingBoards,

    /// The game is currently in progress. No more players can join
    InProgress,

    /// The game has ended
    Completed {
        winners: Vec<String>,
    },
}

#[derive(Debug, Clone)]
pub struct Game {
    pub host: Host,
    pub available_cards: [Card; 25],
    pub players: Vec<Player>,
    pub correct_answers: Vec<CardId>,
    pub start_date: DateTime<Utc>,
    pub state: GameState,
    pub boards: HashMap<Sid, Board>,
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
            boards: HashMap::new(),
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
}

impl Player {
    pub fn new(id: Sid, name: String) -> Self {
        Self { id, name }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Board(pub [Card; 25]);
