mod bot;
mod config;

use bot::client;
use config::loader::load_config;

#[tokio::main]
async fn main() {
    let config = load_config("Config.toml").unwrap();

    client::run(config).await.unwrap();
}
