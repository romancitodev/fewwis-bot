use crate::consts::{TODO_CHANNEL, TODO_TAG};
use crate::helper::db::save_post;
use crate::{Context, Error};

use ::serenity::{
    builder::{Builder, CreateAllowedMentions, CreateEmbed, CreateForumPost, CreateMessage},
    model::Color,
};
use poise::{serenity_prelude as serenity, CreateReply};

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

/// Create a to-do task
#[poise::command(slash_command, category = "Utilities")]
pub async fn create(
    ctx: Context<'_>,
    #[description = "The title of the task"] title: String,
    #[description = "The description of the task"] description: String,
) -> Result<(), Error> {
    let db = &ctx.data().db;
    let reply = CreateReply::new();
    let embed = CreateEmbed::new();
    let post = CreateForumPost::new(
        title.clone(),
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
