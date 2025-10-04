use serde::Serialize;

use crate::model::{Host, LobbyId};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]

pub struct CreateLobbyResponse {
    pub host: Host,
    pub lobby_id: LobbyId,
}
