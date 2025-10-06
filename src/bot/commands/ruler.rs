use poise::command;
use rand::random_range;

use crate::bot::{Context, Error};

#[command(slash_command, prefix_command)]
pub async fn ruler(ctx: Context<'_>) -> Result<(), Error> {
    let length = random_range(0..=32);
    let penis_length = format!("B{}D\n{length}cm", "=".repeat(length));

    ctx.say(penis_length).await?;
    Ok(())
}
