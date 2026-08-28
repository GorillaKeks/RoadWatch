#[tauri::command]
pub fn get_backend_status() -> String {
    "Connected".to_string()
}