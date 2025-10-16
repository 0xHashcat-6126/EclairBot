use poise::{CreateReply, command};
use serenity::all::{
    ChannelId,
    ChannelType,
    CreateEmbed,
    EditChannel,
    GuildId,
    PermissionOverwrite,
    PermissionOverwriteType,
    Permissions,
    RoleId,
    // Dodajemy UserPagination, aby jasno określić typ dla 'None' w bans()
    UserPagination,
};
use serenity::prelude::Context as SerenityContext;
use serenity::prelude::TypeMapKey;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::RwLock;

use crate::bot::{Context, Error};

#[derive(Debug, Clone, Copy, poise::ChoiceParameter, Eq, PartialEq, Hash)]
pub enum StatType {
    #[name = "population"]
    Population,
    #[name = "bots"]
    Bots,
    #[name = "bans"]
    Bans,
}

pub type StatMap = Arc<RwLock<HashMap<(GuildId, ChannelId), StatType>>>;

pub struct StatsKey;
impl TypeMapKey for StatsKey {
    type Value = StatMap;
}

#[command(
    slash_command,
    prefix_command,
    guild_only = true,
    description_localized("en-US", "Adds a statistic to a voice channel")
)]
pub async fn stat(
    ctx: Context<'_>,
    #[description = "Statistic type"] stat_type: StatType,
    #[description = "Channel ID or #mention"] channel_input: String,
) -> Result<(), Error> {
    let is_slash = ctx.prefix().is_empty();

    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            ctx.send(
                CreateReply::default()
                    .content("❌ This command can only be used in a server.")
                    .ephemeral(true),
            )
            .await?;
            return Ok(());
        }
    };

    let channel_id = if let Ok(id) = channel_input.parse::<u64>() {
        ChannelId::new(id)
    } else if let Some(id) = channel_input
        .trim()
        .strip_prefix("<#")
        .and_then(|s| s.strip_suffix('>'))
        .and_then(|s| s.parse::<u64>().ok())
    {
        ChannelId::new(id)
    } else {
        ctx.send(
            CreateReply::default()
                .content("❌ Invalid channel or ID.")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    };

    let channel = match guild_id
        .channels(&ctx.http())
        .await?
        .get(&channel_id)
        .cloned()
    {
        Some(c) => c,
        None => {
            ctx.send(
                CreateReply::default()
                    .content("❌ Channel not found or bot cannot access it.")
                    .ephemeral(true),
            )
            .await?;
            return Ok(());
        }
    };

    let count = if stat_type == StatType::Bans {
        // POPRAWKA BŁĘDU: Dodanie argumentów (UserPagination, limit)
        // Linia oryginalnie zgłaszająca błąd: 89
        match guild_id
            .bans(
                &ctx.serenity_context(),
                None as Option<UserPagination>,
                None,
            )
            .await
        {
            Ok(bans) => bans.len(),
            Err(_) => {
                ctx.send(
                    CreateReply::default()
                        .content("❌ I do not have permission to view bans.")
                        .ephemeral(true),
                )
                .await?;
                return Ok(());
            }
        }
    } else {
        get_stat_value_serenity(&ctx.serenity_context(), guild_id, stat_type).await
    };

    let new_name = format!("{:?}: {}", stat_type, count);

    let edit = EditChannel::default()
        .name(new_name.clone())
        .kind(ChannelType::Voice)
        .permissions(vec![PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::CONNECT,
            kind: PermissionOverwriteType::Role(RoleId::new(guild_id.get())),
        }]);

    channel.id.edit(&ctx.http(), edit).await?;

    let stats = {
        let data_read = ctx.serenity_context().data.read().await;
        if let Some(s) = data_read.get::<StatsKey>() {
            s.clone()
        } else {
            drop(data_read);
            let s = Arc::new(RwLock::new(HashMap::new()));
            ctx.serenity_context()
                .data
                .write()
                .await
                .insert::<StatsKey>(s.clone());
            s
        }
    };

    stats
        .write()
        .await
        .insert((guild_id, channel.id), stat_type);

    ctx.send(
        CreateReply {
            ephemeral: Some(is_slash),
            ..Default::default()
        }
        .embed(
            CreateEmbed::new()
                .title("📊 Statistic updated")
                .description(format!(
                    "Channel `{}` was updated to `{}`",
                    channel.name, new_name
                ))
                .color(0x00BFFF),
        ),
    )
    .await?;

    Ok(())
}

pub async fn get_stat_value_serenity(
    serenity_ctx: &SerenityContext,
    guild_id: GuildId,
    stat_type: StatType,
) -> usize {
    match stat_type {
        StatType::Population => {
            // Metoda members() również wymaga argumentów paginacji i limitu
            match guild_id.members(serenity_ctx, None, None).await {
                Ok(members) => members.iter().filter(|m| !m.user.bot).count(),
                Err(_) => 0,
            }
        }
        StatType::Bots => {
            // Metoda members() również wymaga argumentów paginacji i limitu
            match guild_id.members(serenity_ctx, None, None).await {
                Ok(members) => members.iter().filter(|m| m.user.bot).count(),
                Err(_) => 0,
            }
        }
        StatType::Bans => {
            // POPRAWKA BŁĘDU: Dodanie argumentów (UserPagination, limit)
            match guild_id
                .bans(serenity_ctx, None as Option<UserPagination>, None)
                .await
            {
                Ok(bans) => bans.len(),
                Err(_) => 0,
            }
        }
    }
}

pub async fn start_stat_updater(serenity_ctx: Arc<SerenityContext>, stats: StatMap) {
    tokio::spawn(async move {
        loop {
            let data = stats.read().await;
            for ((guild_id, channel_id), stat_type) in data.iter() {
                let count = get_stat_value_serenity(&serenity_ctx, *guild_id, *stat_type).await;
                let new_name = format!("{:?}: {}", stat_type, count);
                let _ = channel_id
                    .edit(&serenity_ctx.http, EditChannel::default().name(new_name))
                    .await;
            }
            drop(data);
            tokio::time::sleep(Duration::from_secs(300)).await;
        }
    });
}

pub async fn on_member_change(serenity_ctx: &SerenityContext, guild_id: GuildId) {
    let stats = {
        let data_read = serenity_ctx.data.read().await;
        if let Some(s) = data_read.get::<StatsKey>() {
            s.clone()
        } else {
            return;
        }
    };

    let stats_map = stats.read().await;
    for ((g_id, channel_id), stat_type) in stats_map.iter() {
        if *g_id != guild_id {
            continue;
        }

        let count = get_stat_value_serenity(serenity_ctx, *g_id, *stat_type).await;
        let new_name = format!("{:?}: {}", stat_type, count);
        let _ = channel_id
            .edit(&serenity_ctx.http, EditChannel::default().name(new_name))
            .await;
    }
}
