use crate::{Context, Error};
use ::serenity::{
    all::ForumTagId,
    builder::{
        Builder, CreateAllowedMentions, CreateEmbed, CreateForumPost, CreateMessage, EditThread,
    },
    model::Color,
};
use poise::{serenity_prelude as serenity, CreateReply};

use poise::ChoiceParameter;
use serenity::ChannelType;
use std::num::NonZeroU64;
use tracing::error;

/// Shortcut to create a to-do task
#[poise::command(context_menu_command = "Create todo", category = "Utilities")]
pub async fn create_ctx_menu(ctx: Context<'_>, msg: serenity::Message) -> Result<(), Error> {
    let content = msg.content.clone();
    let reply = CreateReply::new();
    let embed = CreateEmbed::new();
    let post = CreateForumPost::new(
        content,
        CreateMessage::new()
            .content(format!("created by {}", ctx.author().clone()))
            .allowed_mentions(CreateAllowedMentions::default().empty_users()),
    )
    .add_applied_tag(1150602743623471215.into())
    .execute(ctx, 1150602558075846816.into())
    .await?;

    ctx.send(
        reply
            .embed(
                embed
                    .color(Color::BLURPLE)
                    .title("✅ To-do created!")
                    .description(format!("> Just go to {}", post)),
            )
            .ephemeral(true),
    )
    .await?;
    msg.delete(ctx).await?;
    Ok(())
}

#[poise::command(slash_command, subcommands("create", "update"), category = "Utilities")]
pub async fn todo(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Create a to-do task
#[poise::command(slash_command, category = "Utilities")]
pub async fn create(
    ctx: Context<'_>,
    #[description = "The title of the task"] title: String,
    #[description = "The description of the task"] description: String,
) -> Result<(), Error> {
    let reply = CreateReply::new();
    let embed = CreateEmbed::new();
    let post = CreateForumPost::new(
        title,
        CreateMessage::new()
            .content(description)
            .allowed_mentions(CreateAllowedMentions::default().empty_users()),
    )
    .add_applied_tag(1150602743623471215.into())
    .execute(ctx, 1150602558075846816.into())
    .await?;

    ctx.send(
        reply
            .embed(
                embed
                    .color(Color::BLURPLE)
                    .title("✅ To-do created!")
                    .description(format!("> Just go to {}", post)),
            )
            .ephemeral(true),
    )
    .await?;
    Ok(())
}

#[repr(u64)]
#[derive(ChoiceParameter, Clone, Copy, PartialEq)]
pub enum Status {
    #[name = "✏ To-do"]
    Todo = 1150602743623471215,
    #[name = "⏳ Working on it"]
    Working = 1150602817585815603,
    #[name = "✅ Finished"]
    Finished = 1150602652904853564,
}

async fn is_forum_thread(ctx: Context<'_>) -> Result<bool, Error> {
    let Ok(serenity::Channel::Guild(channel)) = ctx.channel_id().to_channel(ctx).await else {
        error!("❌ Cannot fetch the guild channel...");
        return Err("Error fetching guild channel".into());
    };

    let Ok(serenity::Channel::Guild(parent_id)) = channel
        .parent_id
        .ok_or("You must be in a forum thread.")?
        .to_channel(ctx)
        .await
    else {
        return Err("You must be in a guild channel.".into());
    };

    if (parent_id.kind != ChannelType::Forum)
        | !matches!(
            channel.kind,
            ChannelType::PublicThread | ChannelType::PrivateThread
        )
    {
        return Err("You must be in a forum post.".into());
    }

    Ok(true)
}

/// Change the status of any task
#[poise::command(slash_command, check = "is_forum_thread", category = "Utilities")]
pub async fn update(
    ctx: Context<'_>,
    #[description = "the new tag to apply to the task"] status: Status,
) -> Result<(), Error> {
    let Ok(serenity::Channel::Guild(mut channel)) = ctx.channel_id().to_channel(ctx).await else {
        error!("❌ Cannot fetch the guild channel...");
        return Err("Error fetching guild channel.".into());
    };

    let reply = CreateReply::new();
    let embed = CreateEmbed::new();

    if channel.applied_tags.contains(&serenity::ForumTagId(
        NonZeroU64::new(status as u64).unwrap(),
    )) {
        return Err("You can't set the same tag.".into());
    }

    ctx.send(
        reply
            .embed(embed.color(Color::FOOYOO).title("✅ To-do updated!"))
            .ephemeral(true),
    )
    .await?;

    channel
        .edit_thread(
            ctx,
            EditThread::default()
                .applied_tags([ForumTagId::from(status as u64)])
                .archived(status == Status::Finished),
        )
        .await?;
    Ok(())
}
