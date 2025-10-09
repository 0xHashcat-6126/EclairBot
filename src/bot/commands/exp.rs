use crate::bot::{Context, Error};
use crate::services::database::models;
use poise::{CreateReply, command};
use serenity::all::{CreateEmbed, Member};

#[command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Check member exp")
)]
pub async fn exp(
    ctx: Context<'_>,
    #[description = "Member to check"] member: Option<Member>,
) -> Result<(), Error> {
    let member_id = match member {
        Some(m) => m.user.id,
        None => ctx.author().id,
    };

    let member = models::member::get_member(&ctx.data().pool, member_id.into()).await?;

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("🥇 Experience:")
                .description(format!("Exp: {}", member.exp))
                .color(0x00FF00),
        ),
    )
    .await?;

    Ok(())
}
