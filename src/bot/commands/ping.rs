use crate::bot::{Context, Error};
use poise::{CreateReply, command};
use serenity::all::CreateEmbed;
use tokio::time::Instant;

#[command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Ping the bot")
)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = Instant::now();
    ctx.http().get_current_user().await?;
    let latency = start.elapsed().as_millis();

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("🏓 Pong!")
                .description(format!("Latency: {} ms", latency))
                .color(0x00FF00)
        ),
    )
    .await?;

    Ok(())
}
