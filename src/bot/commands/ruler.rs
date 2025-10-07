use poise::command;
use rand::random_range;

use crate::bot::{Context, Error};

#[command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Measure your penis length")
)]
pub async fn ruler(ctx: Context<'_>) -> Result<(), Error> {
    let length = random_range(0..=32);
    let penis_length = format!("B{}D\n{length}cm", "=".repeat(length));

    ctx.reply(penis_length).await?;
    Ok(())
}
