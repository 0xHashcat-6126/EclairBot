use crate::bot::{Context, Error};
use crate::utils::role::has_any_role;
use poise::{CreateReply, command};
use serenity::all::{CreateEmbed, Member};

#[command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Free a member"),
    guild_only = true
)]
pub async fn unjail(
    ctx: Context<'_>,
    #[description = "Member to let free"] member: Member,
) -> Result<(), Error> {
    let moderator_roles = ctx
        .author_member()
        .await
        .map_or(vec![], |m| m.roles.clone());

    if has_any_role(&moderator_roles, &ctx.data().config.roles.jail_perm_role) {
        member
            .remove_role(&ctx, &ctx.data().config.roles.jail_role)
            .await?;

        ctx.send(
            CreateReply::default().embed(
                CreateEmbed::new()
                    .title("⛓️‍💥 Unjail")
                    .field(
                        format!("{} unjailed!", member.user.name),
                        "bla bla bla>",
                        false,
                    )
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
                        format!("Cannot unjail user {}.", member.user.name),
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
