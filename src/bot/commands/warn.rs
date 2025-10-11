use crate::bot::{Context, Error};
use crate::services::database::models;
use crate::services::database::models::member::MemberData;
use crate::utils::role::has_any_role;
use poise::{command, CreateReply};
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
        let member_id = i64::from(member.user.id);
        let moderator_id = i64::from(ctx.author().id);

        // ✅ Upewniamy się, że użytkownik i moderator istnieją w tabeli members
        MemberData::ensure_exists(&ctx.data().pool, member_id).await?;
        MemberData::ensure_exists(&ctx.data().pool, moderator_id).await?;

        // Tworzymy i zapisujemy ostrzeżenie
        let warn = models::warn::new(member_id, moderator_id, reason.clone());
        warn.insert(&ctx.data().pool).await?;

        // Wysyłamy embed z potwierdzeniem
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

    // Brak uprawnień — wysyłamy embed z błędem
    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("🛑 ERROR")
                .field(
                    format!("Cannot warn user {}.", member.user.name),
                    "Reason: No permissions",
                    false,
                )
                .thumbnail(member.face())
                .color(0xFF0000),
        ),
    )
    .await?;

    Ok(())
}
