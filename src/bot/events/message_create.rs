use crate::bot::Error;
use crate::bot::client::Data;
use crate::bot::events::actions;
use serenity::all::{Context, Message};

use crate::features::autoreply;

pub async fn message_create(ctx: &Context, data: &Data, msg: &Message) -> Result<(), Error> {
    actions::add_exp::add_exp(data, msg).await?;
    Ok(())
}
