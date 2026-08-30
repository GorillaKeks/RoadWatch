use crate::models::AppSettings;
use crate::services::settings::SettingsStore;

#[tauri::command]
pub fn get_settings() -> Result<AppSettings, String> {
    SettingsStore::load()
}

#[tauri::command]
pub fn save_settings(settings: AppSettings) -> Result<AppSettings, String> {
    let trimmed_vtc_id = settings.vtc_id.trim();
    let trimmed_truckersmp_id = settings.truckersmp_id.trim();

    if trimmed_vtc_id.is_empty() {
        return Err("VTC-ID must not be empty.".to_string());
    }

    if !trimmed_vtc_id
        .chars()
        .all(|character| character.is_ascii_digit())
    {
        return Err("VTC-ID must contain numbers only.".to_string());
    }

    if trimmed_truckersmp_id.is_empty() {
        return Err("TruckersMP-ID must not be empty.".to_string());
    }

    if !trimmed_truckersmp_id
        .chars()
        .all(|character| character.is_ascii_digit())
    {
        return Err("TruckersMP-ID must contain numbers only.".to_string());
    }

    let normalized = AppSettings {
        vtc_id: trimmed_vtc_id.to_string(),
        truckersmp_id: trimmed_truckersmp_id.to_string(),
    };

    SettingsStore::save(&normalized)?;

    Ok(normalized)
}
