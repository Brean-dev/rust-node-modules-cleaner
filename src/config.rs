use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct RuleSet {
    pub patterns: Vec<String>,
    #[allow(dead_code)]
    pub ignore: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(rename = "$default")]
    #[allow(dead_code)]
    pub default: String,
    #[serde(flatten)]
    pub rules: HashMap<String, RuleSet>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaternHits {
    pub patterns: Vec<String>,
}

// Function to read and parse patterns from the JSON file
pub fn read_patterns() -> Result<Config, Box<dyn Error>> {
    let data = include_str!("../patterns.json");
    let config: Config = serde_json::from_str(data)?;
    Ok(config)
}
