use serenity::all::{Context, Member};
use crate::bot::client::Data;
use crate::services::database::models;
use crate::bot::Error;

pub async fn member_init(ctx: &Context, data: &Data, member: &Member) -> Result<(), Error> {
    let member_id = member.user.id.get() as i64;

    let member = models::Member::new(member_id);
    let economy = models::Economy::new(member_id);

    member.insert(&data.pool).await?;
    economy.insert(&data.pool).await?;

    Ok(())
}
