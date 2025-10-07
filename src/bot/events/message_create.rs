use serenity::all::{Context, Message};

use crate::bot::Error;
use crate::bot::events::actions;

pub async fn message_create(ctx: &Context, msg: &Message) -> Result<(), Error> {
    match msg.content.to_lowercase().as_str() {
        "git" => actions::git::git(ctx, msg).await,
        _ => Ok(()),
    }
}
