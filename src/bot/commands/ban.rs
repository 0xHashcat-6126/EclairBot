use crate::bot::{Context, Error};
use poise::command;
use serenity::all::User;

#[command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Ban a member from the server"),
    guild_only = true,
    required_permissions = "BAN_MEMBERS"
)]
pub async fn ban(
    ctx: Context<'_>,
    #[description = "User to ban"] user: User,
    #[description = "Reason for the ban"] reason: String,
    #[description = "Number of days of messages to delete (optional, 0-7)"]
    #[min = 0]
    #[max = 7]
    delete_days: Option<u8>,
) -> Result<(), Error> {
    let guild_id = ctx
        .guild_id()
        .ok_or("This command can be used only in guilds.")?;

    let days = delete_days.unwrap_or(0);

    guild_id
        .ban_with_reason(&ctx, user.id, days, &reason)
        .await?;

    let msg_text = if days == 0 {
        format!("🔨 Banned **{}**: {}", user.name, reason)
    } else {
        format!(
            "🔨 Banned **{}** (deleted {} days of messages): {}",
            user.name, days, reason
        )
    };

    ctx.say(msg_text).await?;

    Ok(())
}
