use poise::{Framework, FrameworkOptions, PrefixFrameworkOptions, builtins};
use serenity::all::GatewayIntents;
use std::error::Error;

use crate::bot::commands;
use crate::config::models::Config;

pub struct Data {}

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let commands = vec![
        commands::ban::ban(),
        commands::kick::kick(),
        commands::ping::ping(),
        commands::ruler::ruler(),
        commands::unban::unban(),
    ];

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands,
            prefix_options: PrefixFrameworkOptions {
                prefix: Some(config.bot.prefix),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::Client::builder(config.bot.token, GatewayIntents::all())
        .framework(framework)
        .await;

    client?.start().await?;
    Ok(())
}
