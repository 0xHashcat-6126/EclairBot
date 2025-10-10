mod bot;
mod config;
mod services;
mod features;
mod util;

use bot::client;
use config::loader::load_config;
use dotenv::dotenv;
use std::path::Path;

use crate::services::database::db;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = load_config("Config.toml").unwrap();
    let path = Path::new(&config.bot.database_name);

    if !path.exists() {
        db::init(&config.bot.database_name).await.unwrap();
    }

    let pool = db::create_pool(&config.bot.database_name).await.unwrap();

    client::run(config, pool).await.unwrap();
}
