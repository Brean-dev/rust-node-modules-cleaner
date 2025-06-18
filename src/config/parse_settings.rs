use simple_config_parser::ConfigError;
use std::collections::HashMap;
use std::path::Path;
use std::{env, fs};

use once_cell::sync::Lazy;
use std::sync::Mutex;

//pub static CONFIG_VALUES: Lazy<HashMap<String, String>> = Lazy::new(HashMap<String, String>);
pub static CONFIG_VALUES: Lazy<Mutex<HashMap<String, String>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn parse_config() -> Result<(), ConfigError> {
    let home_dir = env::var("HOME").unwrap().to_string();
    let user_config = format!("{}/.config/node-module-cleaner/settings.cfg", home_dir);

    let config_paths = [
        "/etc/node-module-cleaner/settings.cfg",
        "./settings.cfg",
        &user_config,
    ];

    for path in &config_paths {
        if Path::new(path).exists() {
            let content = fs::read_to_string(path);
            //let mut config_map: HashMap<String, String> = HashMap::new();

            for line in content.unwrap().lines() {
                let line = line.trim();

                //Skipping the comments in the cfg file

                if line.starts_with("#") || line.is_empty() {
                    continue;
                }

                if let Some((key, value)) = line.split_once('=') {
                    CONFIG_VALUES
                        .lock()
                        .unwrap()
                        .insert(key.trim().to_string(), value.trim().to_string());
                }
            }
            return Ok(());
        }
    }

    Err(ConfigError::NoFileDefined)
}

pub fn get_setting(key: &str) -> Option<String> {
    CONFIG_VALUES.lock().unwrap().get(key).cloned()
}

pub fn get_all_settings() -> Vec<(String, String)> {
    CONFIG_VALUES
        .lock()
        .unwrap()
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}
