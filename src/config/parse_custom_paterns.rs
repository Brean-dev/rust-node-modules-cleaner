use std::path::Path;
use std::{env, fs};

use serde_json::{Value, from_str};

//pub static PATTERNS_VALUE: Lazy<Mutex<HashMap<String, Vec<String>>>> =   Lazy::new(|| Mutex::new(HashMap::new()));

pub fn get_default_patterns() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let home_dir = env::var("HOME")?;
    let user_config = format!("{}/.config/node-module-cleaner/patterns.json", home_dir);

    let config_paths = ["/etc/node-module-cleaner/patterns.json", &user_config];

    for path in &config_paths {
        if Path::new(path).exists() {
            let content = fs::read_to_string(path)?;
            let json_value: Value = from_str(&content)?;

            if let Some(obj) = json_value.as_object() {
                // Get the default pattern type (should be "safe")
                let default_type = obj
                    .get("$default")
                    .and_then(|v| v.as_str())
                    .unwrap_or("safe");

                // Get patterns from the default group
                if let Some(default_group) = obj.get(default_type) {
                    if let Some(pattern_obj) = default_group.as_object() {
                        if let Some(patterns_array) = pattern_obj.get("patterns") {
                            if let Some(patterns) = patterns_array.as_array() {
                                let pattern_strings: Vec<String> = patterns
                                    .iter()
                                    .filter_map(|p| p.as_str())
                                    .map(|s| s.to_string())
                                    .collect();

                                return Ok(pattern_strings);
                            }
                        }
                    }
                }
            }
        }
    }

    Err("Custom patterns file not found".into())
}
