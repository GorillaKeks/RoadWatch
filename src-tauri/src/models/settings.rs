use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub vtc_id: String,
    #[serde(default)]
    pub truckersmp_id: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            vtc_id: String::new(),
            truckersmp_id: String::new(),
        }
    }
}
