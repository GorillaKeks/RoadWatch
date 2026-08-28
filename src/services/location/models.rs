use serde::{Deserialize, Serialize};

/// Supported games for location resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Game {
    Ets2,
    Ats,
}

/// A city entry used for nearest-city location resolution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct City {
    pub name: String,
    pub region: String,
    pub country: String,
    pub x: f64,
    pub y: f64,
}

/// Result of resolving a map position.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationResult {
    pub city: Option<String>,
    pub region: Option<String>,
    pub country: Option<String>,
    pub distance: Option<f64>,
}

impl LocationResult {
    pub fn unknown() -> Self {
        Self {
            city: None,
            region: None,
            country: None,
            distance: None,
        }
    }
}