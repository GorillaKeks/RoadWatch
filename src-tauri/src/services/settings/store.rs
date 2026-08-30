use std::fs;
use std::path::PathBuf;

use crate::models::AppSettings;

pub struct SettingsStore;

impl SettingsStore {
    fn settings_path() -> Result<PathBuf, String> {
        let base_dir = dirs::config_dir()
            .ok_or_else(|| "Could not determine configuration directory.".to_string())?;

        let app_dir = base_dir.join("RoadWatch");

        fs::create_dir_all(&app_dir)
            .map_err(|error| format!("Could not create settings directory: {error}"))?;

        Ok(app_dir.join("settings.json"))
    }

    pub fn load() -> Result<AppSettings, String> {
        let path = Self::settings_path()?;

        if !path.exists() {
            return Ok(AppSettings::default());
        }

        let content = fs::read_to_string(&path)
            .map_err(|error| format!("Could not read settings: {error}"))?;

        serde_json::from_str(&content).map_err(|error| format!("Could not parse settings: {error}"))
    }

    pub fn save(settings: &AppSettings) -> Result<(), String> {
        let path = Self::settings_path()?;

        let content = serde_json::to_string_pretty(settings)
            .map_err(|error| format!("Could not serialize settings: {error}"))?;

        fs::write(path, content).map_err(|error| format!("Could not save settings: {error}"))
    }
}
