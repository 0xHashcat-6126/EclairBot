use serde::Deserialize;
use serenity::all::RoleId;

#[derive(Deserialize)]
pub struct Config {
    pub bot: Bot,
    pub roles: Roles,
}

#[derive(Deserialize)]
pub struct Bot {
    #[serde(default)]
    pub token: String,
    pub prefix: String,
    pub database_name: String,
}

#[derive(Deserialize)]
pub struct Roles {
    pub warn_roles: Vec<RoleId>,
    pub kick_roles: Vec<RoleId>,
}
