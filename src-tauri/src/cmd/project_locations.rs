use crate::config::{load_config, save_config};
use tauri::api::dialog;

#[tauri::command]
pub async fn add_project_location() -> Result<Vec<String>, String> {
    match dialog::blocking::FileDialogBuilder::new().pick_folder() {
        Some(path) => {
            let folder_path = path.to_string_lossy().to_string();

            // Load the current configuration
            let mut config = load_config().map_err(|e| e.to_string())?;

            // Add the folder path to the locations
            config.add_location(folder_path.clone());

            // Save the updated configuration
            save_config(&config).map_err(|e| e.to_string())?;

            Ok(config.locations)
        }
        None => Err("No folder selected".into()),
    }
}

#[tauri::command]
pub async fn delete_project_location(location: String) -> Result<Vec<String>, String> {
    let mut config = load_config().map_err(|e| e.to_string())?;

    // Remove the location from the locations list
    config.remove_location(&location);

    // Save the updated configuration
    save_config(&config).map_err(|e| e.to_string())?;

    // Return the updated locations array
    Ok(config.locations)
}
