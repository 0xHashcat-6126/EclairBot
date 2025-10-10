use crate::bot::{Context, Error};
use crate::services::database::models;
use crate::utils::role::has_any_role;
use poise::{CreateReply, command};
use serenity::all::{CreateEmbed, Member};

#[command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Warn a member"),
    guild_only = true
)]
pub async fn warn(
    ctx: Context<'_>,
    #[description = "Member to warn"] member: Member,
    #[description = "Reason for the warn"] reason: String,
) -> Result<(), Error> {
    let moderator_roles = ctx
        .author_member()
        .await
        .map_or(vec![], |m| m.roles.clone());

    if has_any_role(&moderator_roles, &ctx.data().config.roles.warn_roles) {
        let warn = models::warn::new(
            i64::from(member.user.id),
            i64::from(ctx.author().id),
            reason.clone(),
        );
        warn.insert(&ctx.data().pool).await?;

        ctx.send(
            CreateReply::default().embed(
                CreateEmbed::new()
                    .title("☝️ Warn")
                    .field(
                        format!("{} warned!", member.user.name),
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

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("🛑 ERROR")
                .field(
                    format!("Cannot warn user {}.", member.user.name),
                    "Reason: No permisions",
                    false,
                )
                .thumbnail(member.face())
                .color(0xFF0000),
        ),
    )
    .await?;

    Ok(())
}
