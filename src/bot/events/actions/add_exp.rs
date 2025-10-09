use crate::bot::Error;
use crate::bot::client::Data;
use crate::services::database::models;
use serenity::all::Message;

pub async fn add_exp(data: &Data, message: &Message) -> Result<(), Error> {
    let member_id = message.author.id.get() as i64;

    let mut member = models::member::get_member(&data.pool, member_id).await?;
    member.add_exp(&data.pool, 1).await?;

    Ok(())
}
