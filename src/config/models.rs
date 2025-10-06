use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub bot: Bot,
}

#[derive(Deserialize)]
pub struct Bot {
    pub token: String,
    pub prefix: String,
}
