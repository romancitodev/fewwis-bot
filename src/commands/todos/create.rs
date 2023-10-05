use crate::consts::{OWNER_GUILD, TODO_CHANNEL, TODO_TAG};
use crate::{ApplicationContext, Context, Error};
use ::serenity::collector::ComponentInteractionCollector;
use ::serenity::{
    all::ForumTagId,
    builder::{
        Builder, CreateAllowedMentions, CreateEmbed, CreateForumPost, CreateMessage, EditThread,
    },
    model::Color,
};
use poise::{serenity_prelude as serenity, CreateReply};
use poise::{ChoiceParameter, Modal};
use serenity::ChannelType;
use std::{num::NonZeroU64, time::Duration};
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
    .add_applied_tag(TODO_TAG.into())
    .execute(ctx, TODO_CHANNEL.into())
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

#[poise::command(
    slash_command,
    subcommands("create", "update", "delete", "steps"),
    check = "on_private_guild",
    category = "Utilities"
)]
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
    .add_applied_tag(TODO_TAG.into())
    .execute(ctx, TODO_CHANNEL.into())
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

async fn is_forum_post(ctx: Context<'_>) -> Result<bool, Error> {
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

async fn on_private_guild(ctx: Context<'_>) -> Result<bool, Error> {
    if !ctx
        .guild_id()
        .unwrap()
        .as_inner()
        .eq(&NonZeroU64::new(OWNER_GUILD).unwrap())
    {
        Err("Invalid guild".into())
    } else {
        Ok(true)
    }
}

/// Change the status of any task
#[poise::command(slash_command, check = "is_forum_post", category = "Utilities")]
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

    let forum_tag_id = ForumTagId::from(status as u64);

    if channel.applied_tags.contains(&forum_tag_id) {
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
                .applied_tags([forum_tag_id])
                .archived(status == Status::Finished),
        )
        .await?;
    Ok(())
}

/// Delete a task
#[poise::command(slash_command, check = "is_forum_post", category = "Utilities")]
pub async fn delete(
    ctx: Context<'_>,
    #[description = "Skip the button confirmation."] force: Option<bool>,
) -> Result<(), Error> {
    let Ok(serenity::Channel::Guild(channel)) = ctx.channel_id().to_channel(ctx).await else {
        error!("❌ Cannot fetch the guild channel...");
        return Err("Error fetching guild channel.".into());
    };

    let force = force.unwrap_or_default();

    let reply = CreateReply::new();
    let embed = CreateEmbed::new();
    let row = serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new("confirm")
            .style(serenity::ButtonStyle::Danger)
            .label("Yes")
            .emoji('✅'),
        serenity::CreateButton::new("cancel")
            .style(serenity::ButtonStyle::Secondary)
            .label("No")
            .emoji('❌'),
    ]);

    if !force {
        let response = ctx
            .send(
                reply
                    .clone()
                    .embed(
                        embed
                            .clone()
                            .color(Color::RED)
                            .title("🤚 Wait. Are you sure to delete the task?")
                            .description("> This operation is not reversible."),
                    )
                    .components(vec![row])
                    .ephemeral(true),
            )
            .await?;
        if let Some(interaction) = response
            .message()
            .await?
            .await_component_interactions(ctx.serenity_context().shard.clone())
            .author_id(ctx.author().id)
            .message_id(response.message().await?.id)
            .timeout(Duration::from_secs(60))
            .await
        {
            match interaction.data.custom_id.as_str() {
                "cancel" => {
                    response.delete(ctx).await?;
                }
                "confirm" => {
                    channel.delete(ctx).await?;
                }
                _ => unreachable!(),
            };
        } else {
            response.edit(ctx, reply
                .embed(
                    embed
                        .color(Color::RED)
                        .title("⌛ Oops... Timeout expired!")
                        .description("> You did not interact with any button, so the action was cancelled."),
                ).components(vec![])
                .ephemeral(true)).await?;
        }
    } else {
        channel.delete(ctx).await?;
    }

    Ok(())
}

#[derive(Debug, poise::Modal)]
#[name = "First"]
struct TaskModal {
    #[name = "Milestone"]
    #[placeholder = "Fill this with your milestone"]
    #[min_length = 1]
    #[paragraph]
    task_list: String,
}

#[poise::command(
    slash_command,
    name_localized("es-ES", "pasos"),
    description_localized("es-ES", "Establece los pasos a seguir dentro de la tarea"),
    category = "Utilities"
)]
pub async fn steps(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    let data = TaskModal::execute(ctx).await?.unwrap();
    ctx.say(format!("{:?}", data)).await?;
    Ok(())
}
