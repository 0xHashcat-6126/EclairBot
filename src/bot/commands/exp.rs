use crate::bot::{Context, Error};
use crate::services::database::models;
use poise::{CreateReply, command};
use serenity::all::{CreateEmbed, Member};

use crate::features::level;

#[command(
    slash_command,
    prefix_command,
    aliases("lvl"),
    description_localized("en-US", "Check member exp"),
    guild_only = true
)]
pub async fn exp(
    ctx: Context<'_>,
    #[description = "Member to check"] member: Option<Member>,
) -> Result<(), Error> {
    let member_id = member.map_or(ctx.author().id, |m| m.user.id);
    let member_data = models::member::get_member(&ctx.data().pool, member_id.into()).await?;

    let exp = member_data.exp;
    let level = level::xp_to_level(exp, /* TODO: placeholder */ 100);
    let remaining_exp = exp - level::level_to_xp(level, /* TODO: placeholder */ 100);
    let exp_for_next_level = level::level_to_xp(level + 1, /* TODO: placeholder */ 100);

    let bar = level::make_progress_bar(exp, exp_for_next_level, 12);

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("🥇 Experience:")
                .field(
                    &ctx.author().name,
                    format!(
                        "Level: {level} ({exp}exp)\nRemaining experience points: {remaining_exp}\nNeeded: {}\n{bar}",
                        exp_for_next_level - remaining_exp
                    ),
                    false,
                )
                .thumbnail(&ctx.author().face())
                .color(0x00FF00),
        ),
    )
    .await?;

    Ok(())
}
