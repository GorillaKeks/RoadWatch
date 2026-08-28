use serde::{Deserialize, Serialize};

use super::LivePosition;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum GameType {
    Ets2,
    Ats,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PlayerStatus {
    Online,
    Offline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub id: u64,
    pub truckersmp_id: u64,
    pub username: String,
    pub role: String,
    pub membership: String,
    pub avatar_url: Option<String>,

    pub status: PlayerStatus,
    pub game: Option<GameType>,
    pub server: Option<String>,

    pub location: Option<super::Location>,

    pub live_position: Option<LivePosition>,

    pub distance: Option<String>,
}