use crate::bot::{Context, Error};
use poise::{CreateReply, command};
use serenity::all::{CreateEmbed, Member};

#[command(
    slash_command,
    prefix_command,
    description_localized("en-US", "See a member's profile picture")
)]
pub async fn face(
    ctx: Context<'_>,
    #[description = "Member to see"] member: Option<Member>,
) -> Result<(), Error> {
    let face = member.map_or(ctx.author().face(), |m| m.face());

    ctx.send(CreateReply::default().embed(CreateEmbed::new().image(face)))
        .await?;

    Ok(())
}
