use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;
use std::{env, fs};

use once_cell::sync::Lazy;
use serde_json::json;

//pub static CONFIG_VALUES: Lazy<HashMap<String, String>> = Lazy::new(HashMap<String, String>);
pub static PATTERNS_VALUE: Lazy<Mutex<HashMap<String, Vec<String>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn parse_config() -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = env::var("HOME").unwrap().to_string();
    let user_config = format!("{}/.config/node-module-cleaner/patterns.json", home_dir);

    let config_paths = [
        "/etc/node-module-cleaner/patterns.json",
        "./patterns.json",
        &user_config,
    ];

    for path in &config_paths {
        if Path::new(path).exists() {
            let _pattern = json!(path);

            return Ok(());
        }
    }

    Err("Custom patterns file not found".into())
}

pub fn read_config(key: &str) -> Option<String> {
    PATTERNS_VALUE.lock().unwrap().get(key).cloned()
}
