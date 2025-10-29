use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinLobbyRequest {
    pub player_name: String,
    pub lobby_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostLobbyRequest {
    pub host_name: String,
    pub cards: [String; 25],
}
