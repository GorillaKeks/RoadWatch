use serde::{Deserialize, Serialize};

use super::{GameType, PlayerStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LivePlayer {
    pub truckersmp_id: u64,
    pub username: String,
    pub status: PlayerStatus,
    pub game: Option<GameType>,
    pub server_id: Option<u64>,
    pub server: Option<String>,
    pub location: Option<super::Location>,
    pub position: Option<LivePosition>,
    pub heading: Option<f64>,
    pub timestamp: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LivePosition {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
}
