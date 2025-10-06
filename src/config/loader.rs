use std::error::Error;
use std::fs;

use crate::config::models::Config;

pub fn load_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let config = toml::from_str(&content)?;

    Ok(config)
}
