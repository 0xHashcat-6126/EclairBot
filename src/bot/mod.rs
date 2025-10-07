use crate::bot::client::Data;

pub mod client;
pub mod commands;
pub mod events;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
