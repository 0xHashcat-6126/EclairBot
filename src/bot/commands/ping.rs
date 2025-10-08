use crate::bot::{Context, Error};
use poise::{CreateReply, command};
use serenity::all::CreateEmbed;

#[command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Ping the bot")
)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let latency = ctx.ping().await;
    let ms = latency.as_millis();

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("🏓 Pong!")
                .description(format!("Latency: {} ms", ms))
                .color(0x00FF00)
                .timestamp(chrono::Utc::now()),
        ),
    )
    .await?;

    Ok(())
}
