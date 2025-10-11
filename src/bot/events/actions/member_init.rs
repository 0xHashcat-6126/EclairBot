use crate::bot::Error;
use crate::bot::client::Data;
use crate::services::database::models::member::MemberData;
use crate::services::database::models::timeout;
use serenity::all::Member;

pub async fn member_init(data: &Data, member: &Member) -> Result<(), Error> {
    let member_id = member.user.id.get() as i64;

    let member = MemberData::new(member_id);
    let timeouts = timeout::new(member_id);

    member.insert(&data.pool).await?;
    timeouts.insert(&data.pool).await?;

    Ok(())
}
