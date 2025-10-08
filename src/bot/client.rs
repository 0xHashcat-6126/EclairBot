use crate::bot::{commands, events};
use crate::config::models::Config;
use poise::{Framework, FrameworkOptions, PrefixFrameworkOptions, builtins};
use serenity::all::{FullEvent, GatewayIntents};
use sqlx::{Pool, Sqlite};
use std::error::Error;

pub struct Data {
    pub pool: Pool<Sqlite>,
}

pub async fn run(config: Config, pool: Pool<Sqlite>) -> Result<(), Box<dyn Error>> {
    let commands = vec![
        commands::ban::ban(),
        commands::kick::kick(),
        commands::help::help(),
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
            event_handler: |ctx, event, _framework, data| {
                Box::pin(async move {
                    match event {
                        FullEvent::Message { new_message } => {
                            events::message::message(ctx, &data, &new_message).await?;
                        }
                        FullEvent::GuildMemberAddition { new_member } => {
                            events::guild_member_addition::guild_member_addition(
                                ctx,
                                &data,
                                &new_member,
                            )
                            .await?;
                        }
                        _ => (),
                    }
                    Ok(())
                })
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { pool })
            })
        })
        .build();

    let client = serenity::Client::builder(config.bot.token, GatewayIntents::all())
        .framework(framework)
        .await;

    client?.start().await?;
    Ok(())
}
