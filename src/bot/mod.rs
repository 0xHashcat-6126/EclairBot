use crate::bot::client::Data;

pub mod client;
pub mod commands;
pub mod events;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
