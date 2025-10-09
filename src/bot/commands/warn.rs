use poise::{command, CreateReply};
use serenity::all::{CreateEmbed, User};
use crate::services::database::models;
use crate::bot::{Context, Error};
use crate::utils::role::has_any_role;

#[command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Warn a user"),
    guild_only = true,
)]
pub async fn warn(
    ctx: Context<'_>,
    #[description = "User to warn"] user: User,
    #[description = "Reason for the warn"] reason: String,
) -> Result<(), Error> {
    let member_roles = ctx.author_member().await.map_or(vec![], |m| m.roles.clone());

    if has_any_role(&member_roles, &ctx.data().config.roles.warn_roles) {
        let warn = models::warn::new(i64::from(user.id), i64::from(ctx.author().id), reason.clone());
        warn.insert(&ctx.data().pool).await?;

        ctx.send(
            CreateReply::default().embed(
                CreateEmbed::new()
                    .title("☝️ Warn")
                    .field(format!("{} warned!", ctx.author().name), format!("Reason: {reason}"), false)
                    .thumbnail(&ctx.author().face())
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
                .field(format!("Cannot warn user {}.", ctx.author().name), "Reason: No permisions", false)
                .thumbnail(&ctx.author().face())
                .color(0xFF0000),
        ),
    )
        .await?;

    Ok(())
}
