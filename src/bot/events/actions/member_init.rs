use crate::bot::Error;
use crate::bot::client::Data;
use serenity::all::{Context, Member};

pub async fn member_init(ctx: &Context, data: &Data, member: &Member) -> Result<(), Error> {
    todo!();

    Ok(())
}
