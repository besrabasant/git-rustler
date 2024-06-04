use dirs;
use serde::{ Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use toml::{ser,from_str};
use std::io::prelude::*;

const CONFIG_PATH: &str = ".config/git-rustler/config.toml";

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub locations: Vec<String>,
}


pub fn load_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().unwrap();
    let home_path = PathBuf::from(home_dir);

    // Construct the config file path by joining the home directory path and CONFIG_PATH
    let config_abs_path = home_path.join(CONFIG_PATH);

    if !Path::new(config_abs_path.as_path()).exists() {
        // Create all directories in the path if they don't exist
        let mut new_config_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(config_abs_path.as_path())?;

        // Create an empty AppConfig instance
        let empty_config = AppConfig { locations: Vec::new() };

        // Serialize the empty AppConfig to TOML string
        let toml_string = ser::to_string(&empty_config)?;

        // Write the serialized TOML string to the file
        new_config_file.write_all(toml_string.as_bytes())?;
    }

    // Read the file contents
    let config_file = File::open(config_abs_path.as_path())?;
    let mut buf_reader = BufReader::new(config_file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    // Parse the TOML content into AppConfig struct
    let config = from_str::<AppConfig>(&contents)?;

    Ok(config)
}
