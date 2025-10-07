mod bot;
mod config;
mod services;

use bot::client;
use config::loader::load_config;

#[tokio::main]
async fn main() {
    let config = load_config("Config.toml").unwrap();
    services::database::db::init(&config.bot.database_name).await.unwrap();

    client::run(config).await.unwrap();
}
