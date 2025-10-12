use crate::bot::{Context, Error};
use crate::services::database::models::warn::WarnData;
use crate::utils::role::has_any_role;
use poise::{CreateReply, command};
use serenity::all::{CreateEmbed, Member};

#[derive(Debug, poise::ChoiceParameter)]
pub enum ListType {
    Warn,
}

#[command(
    slash_command,
    prefix_command,
    guild_only = true,
    description_localized("en-US", "List moderation data")
)]
pub async fn list(
    ctx: Context<'_>,
    #[description = "Typ danych do wyświetlenia"] list_type: ListType,
    #[description = "Użytkownik, którego dane chcesz zobaczyć"] member: Member,
) -> Result<(), Error> {
    let is_slash = ctx.prefix().is_empty();

    let moderator_roles = ctx
        .author_member()
        .await
        .map_or(vec![], |m| m.roles.clone());

    if !has_any_role(&moderator_roles, &ctx.data().config.roles.warn_roles) {
        ctx.send(
            CreateReply {
                ephemeral: Some(is_slash),
                ..CreateReply::default()
            }
            .embed(
                CreateEmbed::new()
                    .title("🛑 ERROR")
                    .field(
                        format!(
                            "Brak uprawnień do przeglądania danych użytkownika {}.",
                            member.user.name
                        ),
                        "Musisz mieć odpowiednią rolę moderacyjną.",
                        false,
                    )
                    .thumbnail(member.face())
                    .color(0xFF0000),
            ),
        )
        .await?;
        return Ok(());
    }

    match list_type {
        ListType::Warn => {
            let warns = WarnData::get_by_user(&ctx.data().pool, i64::from(member.user.id)).await?;

            if warns.is_empty() {
                ctx.send(
                    CreateReply {
                        ephemeral: Some(is_slash),
                        ..CreateReply::default()
                    }
                    .embed(
                        CreateEmbed::new()
                            .title("✅ Brak ostrzeżeń")
                            .description(format!(
                                "Użytkownik {} nie ma żadnych ostrzeżeń.",
                                member.user.name
                            ))
                            .thumbnail(member.face())
                            .color(0x00FF00),
                    ),
                )
                .await?;
                return Ok(());
            }

            let mut embed = CreateEmbed::new()
                .title(format!("📋 Ostrzeżenia dla {}", member.user.name))
                .thumbnail(member.face())
                .color(0xFFA500);

            for warn in warns.iter().take(5) {
                embed = embed.field(
                    format!("ID: {}", warn.id),
                    format!(
                        "Moderator: <@{}>\nPowód: {}\nTimestamp: {}",
                        warn.moderator_id, warn.reason, warn.timestamp,
                    ),
                    false,
                );
            }

            ctx.send(
                CreateReply {
                    ephemeral: Some(is_slash),
                    ..CreateReply::default()
                }
                .embed(embed),
            )
            .await?;
        }
    }

    Ok(())
}
