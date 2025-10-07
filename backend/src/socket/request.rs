use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct JoinLobbyRequest {
    pub player_name: String,
    pub lobby_id: Uuid,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostLobbyRequest {
    pub host_name: String,
    pub cards: [String; 25],
}
