use crate::bot::{Context, Error};
use poise::command;
use serenity::all::User;

#[command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Kick a member from the server"),
    guild_only = true,
    required_permissions = "KICK_MEMBERS"
)]
pub async fn kick(
    ctx: Context<'_>,
    #[description = "User to kick"] member: User,
    #[description = "Reason for the kick"] reason: String,
) -> Result<(), Error> {
    let guild_id = ctx
        .guild_id()
        .ok_or("This command can be used only in guilds.")?;
    let member = guild_id.member(ctx, member.id).await?;

    member.kick_with_reason(ctx, &reason).await?;

    ctx.say(format!(
        "👢 Kicked **{}**: {}",
        member.display_name(),
        reason
    ))
    .await?;

    Ok(())
}
