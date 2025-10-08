use crate::bot::Error;
use crate::bot::client::Data;
use serenity::all::{Context, Member};

pub async fn guild_member_addition(
    ctx: &Context,
    data: &Data,
    member: &Member,
) -> Result<(), Error> {
    Ok(())
}
