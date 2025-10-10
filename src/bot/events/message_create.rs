use crate::bot::Error;
use crate::bot::client::Data;
use crate::bot::events::actions;
use serenity::all::{Context, Message};

use crate::features::autoreply::{AutoReplyRule, AutoReplyTarget, AutoReplyDef};
use crate::autoreply;

use lazy_static::lazy_static;

lazy_static! {
    static ref AUTOREPLY_DEFS: &'static [AutoReplyDef] = Box::leak(Box::new([
        autoreply!("git", "hub"),
    ]));
}

pub async fn message_create(ctx: &Context, data: &Data, msg: &Message) -> Result<(), Error> {
    for autoreply_def in AUTOREPLY_DEFS.iter() {
        autoreply_def.handle(&ctx.http, msg).await;
    }    

    actions::add_exp::add_exp(data, msg).await?;
    Ok(())
}
