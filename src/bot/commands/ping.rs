use crate::bot::{Context, Error};
use poise::command;

#[command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Ping the bot")
)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("Pong!").await?;
    Ok(())
}
