use crate::bot::{Context, Error};
use poise::command;
use serenity::all::User;

#[command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Unban a member from the server"),
    guild_only = true,
    required_permissions = "BAN_MEMBERS"
)]
pub async fn unban(
    ctx: Context<'_>,
    #[description = "User to unban"] user: User,
    #[description = "Reason for the unban"] reason: String,
) -> Result<(), Error> {
    let guild_id = ctx
        .guild_id()
        .ok_or("This command can be used only in guilds.")?;

    guild_id
        .unban(&ctx, user.id)
        .await
        .map_err(|e| format!("Failed to unban {}: {}", user.name, e))?;

    ctx.say(format!("✅ Unbanned **{}**: {}", user.name, reason))
        .await?;

    Ok(())
}
