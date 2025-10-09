use crate::bot::{Context, Error};
use poise::command;
use serenity::all::Member;

#[command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Check member exp")
)]
pub async fn exp(
    ctx: Context<'_>,
    #[description = "Member to check"] member: Option<Member>,
) -> Result<(), Error> {
    Ok(())
}
