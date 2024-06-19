use crate::config::{load_config, AppConfig};

#[tauri::command]
pub async fn get_app_config() -> Result<AppConfig, String> {
    load_config().map_err(|e| e.to_string())
}