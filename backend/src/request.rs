use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateLobbyRequest {
    pub host_name: String,
    pub cards: [String; 25],
}
