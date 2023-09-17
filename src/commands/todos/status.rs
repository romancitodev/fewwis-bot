use std::num::NonZeroU64;

use crate::{Context, Error};
use discord::{builder::CreateEmbed, model::Color, ChannelType};
use poise::{serenity_prelude as discord, ChoiceParameter, CreateReply};
use tracing::error;

#[repr(u64)]
#[derive(ChoiceParameter)]
pub enum Status {
    #[name = "✏ To-do"]
    Todo = 1150602743623471215,
    #[name = "⏳ Working on it"]
    Working = 1150602817585815603,
    #[name = "✅ Finished"]
    Finished = 1150602652904853564,
}

async fn is_forum_thread(ctx: Context<'_>) -> Result<bool, Error> {
    let Ok(discord::Channel::Guild(channel)) = ctx.channel_id().to_channel(ctx).await else {
        error!("❌ Cannot fetch the guild channel...");
        return Err("Error fetching guild channel".into());
    };

    let Ok(discord::Channel::Guild(parent_id)) = channel
        .parent_id
        .ok_or("You must be in a forum thread")?
        .to_channel(ctx)
        .await
    else {
        return Err("You must be in a guild channel".into());
    };

    if (parent_id.kind != ChannelType::Forum)
        | !matches!(
            channel.kind,
            ChannelType::PublicThread | ChannelType::PrivateThread
        )
    {
        return Err("You must be in a forum post".into());
    }

    Ok(true)
}

/// Change the status of any task
#[poise::command(slash_command, check = "is_forum_thread", category = "Utilities")]
pub async fn status(ctx: Context<'_>, status: Status) -> Result<(), Error> {
    let Ok(discord::Channel::Guild(channel)) = ctx.channel_id().to_channel(ctx).await else {
        error!("❌ Cannot fetch the guild channel...");
        return Err("Error fetching guild channel".into());
    };

    let reply = CreateReply::new();
    let embed = CreateEmbed::new();

    if channel.applied_tags.contains(&discord::ForumTagId(
        NonZeroU64::new(status as u64).unwrap(),
    )) {
        return Err("You can't set the same tag".into());
    }

    ctx.send(
        reply
            .embed(
                embed
                    .color(Color::BLURPLE)
                    .title("✅ To-do updated!")
                    .description(format!("> Just go to ")),
            )
            .ephemeral(true),
    )
    .await?;
    Ok(())
}
