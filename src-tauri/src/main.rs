
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod cmd;
mod config;

fn main() {
    let _config_result = config::load_config();


    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            cmd::app::get_app_config, 
            cmd::project_locations::add_project_location,
            cmd::project_locations::delete_project_location,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
