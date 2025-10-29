use serde::Serialize;

use crate::model::Card;

#[derive(Debug, Serialize)]
pub struct JoinLobbyAck {
    pub players: Vec<String>,
    pub host: String,
    pub cards: [Card; 25],
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HostLobbyAck {
    pub lobby_id: String,
    pub cards: [Card; 25],
}
