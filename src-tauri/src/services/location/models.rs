use serde::{Deserialize, Serialize};

/// Supported games for location resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Game {
    Ets2,
    Ats,
}

/// A normalized city entry used internally for nearest-city
/// location resolution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct City {
    pub name: String,
    pub region: String,
    pub country: String,
    pub x: f64,
    pub y: f64,
}

/// Original ATS city data format.
///
/// The current ATS city file is a direct JSON array.
#[derive(Debug, Clone, Deserialize)]
pub struct AtsCity {
    pub token: String,

    pub name: String,

    #[serde(rename = "countryToken")]
    pub country_token: String,

    pub population: Option<u64>,

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
