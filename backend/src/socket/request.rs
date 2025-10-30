use serde::Deserialize;

use crate::model::LobbyId;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinLobbyRequest {
    pub player_name: String,
    pub lobby_id: LobbyId,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostLobbyRequest {
    pub host_name: String,
    pub cards: [String; 25],
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoardSubmitRequest {
    pub lobby_id: LobbyId,
    pub cards: [u8; 25],
}
