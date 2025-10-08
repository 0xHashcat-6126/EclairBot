use crate::bot::Error;
use crate::bot::client::Data;
use crate::bot::events::actions;
use serenity::all::{Context, Message};

pub async fn message(ctx: &Context, data: &Data, msg: &Message) -> Result<(), Error> {
    match msg.content.to_lowercase().as_str() {
        "git" => actions::git::git(ctx, msg).await,
        _ => Ok(()),
    }
}
