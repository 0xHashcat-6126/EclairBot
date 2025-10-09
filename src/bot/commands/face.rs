use poise::{command, CreateReply};
use serenity::all::{CreateEmbed, Member};
use crate::bot::{Context, Error};


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

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .image(face)
        ),
    )
        .await?;

    Ok(())
}
