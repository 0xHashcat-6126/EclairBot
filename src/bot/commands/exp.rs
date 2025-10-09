use crate::bot::{Context, Error};
use crate::services::database::models;
use poise::{CreateReply, command};
use serenity::all::{CreateEmbed, Member};

#[command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Check member exp"),
    guild_only = true,
)]
pub async fn exp(
    ctx: Context<'_>,
    #[description = "Member to check"] member: Option<Member>,
) -> Result<(), Error> {
    let member_id = member.map_or(ctx.author().id, |m| m.user.id);
    let member_data = models::member::get_member(&ctx.data().pool, member_id.into()).await?;

    let mut level = 0;
    let mut exp_needed = 64;
    let mut remaining_exp = member_data.exp;

    while remaining_exp >= exp_needed {
        remaining_exp -= exp_needed;
        level += 1;
        exp_needed *= 2;
    }

    let bar_length = 12;
    let filled_length = ((remaining_exp as f64 / exp_needed as f64) * bar_length as f64).round() as usize;
    let bar = format!(
        "{}{}",
        "🟩".repeat(filled_length),
        "⬛".repeat(bar_length - filled_length)
    );

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("🥇 Experience:")
                .field(&ctx.author().name, format!("Level: {level}\nExperience: {remaining_exp}\nNeeded: {}\n{bar}", exp_needed - remaining_exp), false)
                .thumbnail(&ctx.author().face())
                .color(0x00FF00),
        ),
    )
    .await?;

    Ok(())
}
