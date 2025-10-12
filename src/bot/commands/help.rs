use crate::bot::{Context, Error};
use poise::{CreateReply, command};
use serenity::all::CreateEmbed;

#[command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Help for commands")
)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Command to help with"] command: Option<String>,
) -> Result<(), Error> {
    let embed = if let Some(cmd) = command {
        let (title, description, usage) = match cmd.to_lowercase().as_str() {
            "ping" => (
                "Ping Command",
                "Checks if the bot is alive.\n\nIf the bot is alive, it will respond with a 'Pong!'.",
                "`ping`",
            ),
            "ruler" => (
                "Ruler Command",
                "Measures the length of your penis.\n\nDraws a random number from 0 to 32 and display size of your penis in 'B<LENGTH>D' format, with size at the bottom in '<LENGTH>cm' format.\n\nexample:\n B========D\n8cm",
                "`ruler`",
            ),
            _ => (
                "Unknown Command",
                "Could not find help for this command.",
                "Try `help <command>` with a valid command name.",
            ),
        };

        CreateEmbed::new()
            .title(title)
            .description(description)
            .field("Usage", usage, false)
    } else {
        CreateEmbed::new()
            .title("Available Commands")
            .description("List of all available commands:")
            .field("ban", "Bans a user from the server.", false)
            .field("help", "Shows this message, and command usage.", false)
            .field("kick", "Kicks a user from the server.", false)
            .field("ping", "Checks if the bot is alive.", false)
            .field("ruler", "Measures the length of your penis.", false)
            .field("unban", "Unbans a user from the server.", false)
            .field("list", "List data like: warns, bans, mutes.", false)
    };

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}
