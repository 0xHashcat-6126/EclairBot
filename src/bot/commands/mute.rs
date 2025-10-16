use crate::bot::{Context, Error};
use crate::utils::role::has_any_role;
use poise::{CreateReply, command};
use serenity::all::{CreateEmbed, Member};

#[command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Mute a member"),
    guild_only = true
)]
pub async fn mute(
    ctx: Context<'_>,
    #[description = "Member to mute"] member: Member,
) -> Result<(), Error> {
    let moderator_roles = ctx
        .author_member()
        .await
        .map_or(vec![], |m| m.roles.clone());

    if has_any_role(&moderator_roles, &ctx.data().config.roles.mute_perm_roles) {
        member
            .add_role(&ctx, &ctx.data().config.roles.mute_role)
            .await?;

        ctx.send(
            CreateReply::default().embed(
                CreateEmbed::new()
                    .title("🤐 Mute")
                    .field(format!("{} muted!", member.user.name), "Shh...", false)
                    .thumbnail(member.face())
                    .color(0x00FF00),
            ),
        )
        .await?;
    } else {
        ctx.send(
            CreateReply::default().embed(
                CreateEmbed::new()
                    .title("🛑 ERROR")
                    .field(
                        format!("Cannot mute user {}.", member.user.name),
                        "Reason: No permissions",
                        false,
                    )
                    .thumbnail(member.face())
                    .color(0xFF0000),
            ),
        )
        .await?;
    }

    Ok(())
}
