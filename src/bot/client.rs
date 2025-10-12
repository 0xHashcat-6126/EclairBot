use crate::bot::{commands, events};
use crate::config::models::Config;
use poise::{Framework, FrameworkOptions, PrefixFrameworkOptions, builtins};
use serenity::all::{FullEvent, GatewayIntents};
use sqlx::{Pool, Sqlite};
use std::error::Error;

pub struct Data {
    pub pool: Pool<Sqlite>,
    pub config: Config,
}

pub async fn run(config: Config, pool: Pool<Sqlite>) -> Result<(), Box<dyn Error>> {
    let token = config.bot.token.clone();
    let prefix = config.bot.prefix.clone();

    let commands = vec![
        commands::ban::ban(),
        commands::exp::exp(),
        commands::face::face(),
        commands::kick::kick(),
        commands::help::help(),
        commands::ping::ping(),
        commands::ruler::ruler(),
        commands::unban::unban(),
        commands::warn::warn(),
        commands::list::list(),
        commands::stat::stat(),
    ];

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands,
            prefix_options: PrefixFrameworkOptions {
                prefix: Some(prefix),
                ..Default::default()
            },
            event_handler: |ctx, event, _framework, data| {
                Box::pin(async move {
                    match event {
                        FullEvent::Message { new_message } => {
                            events::message_create::message_create(ctx, &data, &new_message)
                                .await?;
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
                Ok(Data { pool, config })
            })
        })
        .build();

    let client = serenity::Client::builder(token, GatewayIntents::all())
        .framework(framework)
        .await;

    client?.start().await?;
    Ok(())
}
