use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub bot: Bot,
}

#[derive(Deserialize)]
pub struct Bot {
    #[serde(default)]
    pub token: String,
    pub prefix: String,
    pub database_name: String,
}
