use crate::bot::Error;
use crate::bot::client::Data;
use serenity::all::{Context, Member};
use crate::bot::events::actions::member_init::member_init;

pub async fn guild_member_addition(
    ctx: &Context,
    data: &Data,
    member: &Member,
) -> Result<(), Error> {
    member_init(ctx, data, member).await.map_err(|e| Error::from(e.to_string()))?;

    Ok(())
}
