use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct JoinLobbyAck {
    pub players: Vec<String>,
    pub host: String,
}
