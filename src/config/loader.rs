use std::error::Error;
use std::{env, fs};

use crate::config::models::Config;

pub fn load_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let content: String = fs::read_to_string(path)?;
    let mut config: Config = toml::from_str(&content)?;

    if config.bot.token.is_empty() || config.bot.token == "<BOT_TOKEN>" {
        config.bot.token = env::var("ECLAIRBOT_TOKEN")
            .expect("Expected bot token in ECLAIRBOT_TOKEN env variable or in config file.");
    }

    Ok(config)
}
