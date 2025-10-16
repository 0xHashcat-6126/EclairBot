use crate::bot::{Context, Error};
use crate::services::database::models;
use crate::utils::role::has_any_role;
use poise::{CreateReply, command};
use serenity::all::{CreateEmbed, Member};

#[command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Kick a member from the server"),
    guild_only = true,
    required_permissions = "KICK_MEMBERS"
)]
pub async fn kick(
    ctx: Context<'_>,
    #[description = "User to kick"] member: Member,
    #[description = "Reason for the kick"] reason: String,
) -> Result<(), Error> {
    let moderator_roles = ctx
        .author_member()
        .await
        .map_or(vec![], |m| m.roles.clone());

    member.kick_with_reason(ctx, &reason).await?;

    if has_any_role(&moderator_roles, &ctx.data().config.roles.kick_perm_roles) {
        let kick = models::kick::new(
            i64::from(member.user.id),
            i64::from(ctx.author().id),
            reason.clone(),
        );
        kick.insert(&ctx.data().pool).await?;

        ctx.send(
            CreateReply::default().embed(
                CreateEmbed::new()
                    .title("🦶 Kick")
                    .field(
                        format!("{} kicked!", member.user.name),
                        format!("Reason: {reason}"),
                        false,
                    )
                    .thumbnail(member.face())
                    .color(0x00FF00),
            ),
        )
        .await?;

        return Ok(());
    }

    Ok(())
}
