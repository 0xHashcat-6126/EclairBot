use serenity::all::{Context, Message};

use crate::bot::Error;

pub async fn git(ctx: &Context, msg: &Message) -> Result<(), Error> {
    msg.channel_id.say(ctx, "hub").await?;

    Ok(())
}
