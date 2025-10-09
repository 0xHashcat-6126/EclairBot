use crate::bot::Error;
use crate::bot::client::Data;
use crate::bot::events::actions::member_init::member_init;
use serenity::all::{Context, Member};

pub async fn guild_member_addition(
    ctx: &Context,
    data: &Data,
    member: &Member,
) -> Result<(), Error> {
    member_init(data, member).await?;

    Ok(())
}
