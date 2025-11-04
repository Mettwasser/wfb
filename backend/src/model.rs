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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    id: u8,
    description: String,
}

impl Card {
    pub fn new(description: String, idx: u8) -> Self {
        Self {
            description,
            id: idx,
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

    /// Players. No more players can join
    CraftingBoards,

    /// The lobby is currently in progress.
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
    pub players: HashMap<Sid, Player>,
    pub start_date: DateTime<Utc>,
    pub state: LobbyState,

    // We only save the card ids. They are enough for checking the winner.
    pub correct_answers: Vec<u8>,
    pub boards: HashMap<Sid, [u8; 25]>,
}

impl Lobby {
    pub fn new(host: Host, available_cards: [Card; 25]) -> Self {
        Self {
            host,
            available_cards,
            players: HashMap::new(),
            correct_answers: Vec::with_capacity(25),
            start_date: Utc::now(),
            state: LobbyState::WaitingForPlayers,
            boards: HashMap::new(),
        }
    }

    pub fn is_host(&self, socket_id: Sid) -> bool {
        self.host.id == socket_id
    }

    pub fn remove_player(&mut self, sid: &Sid) -> Option<Player> {
        self.boards.remove(sid);
        self.players.remove(sid)
    }

    pub fn advance_state(&mut self) -> Result<LobbyState, LastStateReached> {
        self.state = self.state.next_stage().ok_or(LastStateReached)?;
        Ok(self.state)
    }

    pub fn check_winners(&self) -> Vec<&str> {
        let mut winners = Vec::new();

        for (player_id, board) in &self.boards {
            if check_winner_board(*board, &self.correct_answers)
                && let Some(player) = self.players.get(player_id)
            {
                winners.push(player.name.as_str());
            }
        }

        winners
    }
}

fn check_winner_board(board: [u8; 25], correct_answers: &[u8]) -> bool {
    // Check rows
    for row in 0..5 {
        let start = row * 5;
        let row_slice = &board[start..start + 5];
        if row_slice.iter().all(|id| correct_answers.contains(id)) {
            return true;
        }
    }

    // Check columns
    for col in 0..5 {
        if (0..5).all(|row| correct_answers.contains(&board[row * 5 + col])) {
            return true;
        }
    }

    // Check main diagonal (top-left to bottom-right)
    if (0..5).all(|i| correct_answers.contains(&board[i * 5 + i])) {
        return true;
    }

    // Check other diagonal (top-right to bottom-left)
    if (0..5).all(|i| correct_answers.contains(&board[i * 5 + (4 - i)])) {
        return true;
    }

    false
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

#[cfg(test)]
mod tests {
    use crate::model::check_winner_board;

    #[rustfmt::skip]
    const fn sample_board() -> [u8; 25] {
        [
        // Columns
        //   0   1   2   3   4  // rows
             1,  2,  3,  4,  5, // 0
             6,  7,  8,  9, 10, // 1
            11, 12, 13, 14, 15, // 2
            16, 17, 18, 19, 20, // 3
            21, 22, 23, 24, 25, // 4
        ]
    }

    #[test]
    fn row_win_detected() {
        let board = sample_board();
        // row 1 (second row): 6,7,8,9,10 scrambled
        let correct = vec![9, 6, 10, 7, 8];
        assert!(check_winner_board(board, &correct));
    }

    #[test]
    fn column_win_detected() {
        let board = sample_board();
        // column 2 (third column): values 3,8,13,18,23 scrambled
        let correct = vec![18, 3, 23, 8, 13];
        assert!(check_winner_board(board, &correct));
    }

    #[test]
    fn main_diagonal_win_detected() {
        let board = sample_board();
        // main diagonal: values 1,7,13,19,25 scrambled
        let correct = vec![13, 25, 1, 19, 7];
        assert!(check_winner_board(board, &correct));
    }

    #[test]
    fn other_diagonal_win_detected() {
        let board = sample_board();
        // other diagonal: values 5,9,13,17,21 scrambled
        let correct = vec![21, 13, 5, 17, 9];
        assert!(check_winner_board(board, &correct));
    }

    #[test]
    fn no_win_detected() {
        let board = sample_board();
        // scattered answers that don't make any full row/col/diagonal
        let correct = vec![14, 1, 22, 6, 3]; // scrambled scattered set
        assert!(!check_winner_board(board, &correct));
    }
}
