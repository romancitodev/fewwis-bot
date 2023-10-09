use std::fmt::Write;

use crate::{
    helper::db::{add_steps, get_all_steps, get_post},
    types::{ApplicationContext, Error},
};
use ::serenity::builder::{CreateEmbed, EditMessage};
use poise::Modal;

#[derive(Debug, Modal)]
#[name = "To-do List"]
struct TaskModal {
    #[name = "Milestone"]
    #[placeholder = "Fill this with your milestone"]
    #[min_length = 1]
    #[paragraph]
    task_list: String,
}

#[poise::command(
    slash_command,
    name_localized("es-ES", "agregar"),
    description_localized("es-ES", "Establece los pasos a seguir dentro de la tarea"),
    category = "Utilities",
    rename = "add"
)]
pub async fn add(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    let Ok(Some(data)) = TaskModal::execute(ctx).await else {
        ctx.defer_ephemeral().await?;
        ctx.reply("❌ Timeout... you have ran out of time to complete the modal.")
            .await?;
        return Ok(());
    };
    let db = &ctx.data().db;
    let channel_id = ctx.channel_id().get();
    let tasks: Vec<String> = data.task_list.split('\n').map(String::from).collect();
    let post = get_post(db, channel_id).await?;

    add_steps(db, post.id, tasks).await?;
    let tasks = get_all_steps(db, post.id)
        .await?
        .iter()
        .fold(String::new(), |mut acc, task| {
            writeln!(
                &mut acc,
                "> [{}] **{}.** {}",
                if task.completed != 0 { "X" } else { " " },
                task.index,
                task.description
            )
            .unwrap();
            acc
        });

    let message = ctx.channel_id().message(ctx.http(), post.post_id).await?;

    let mut embeds = message
        .embeds
        .iter()
        .cloned()
        .map(CreateEmbed::from)
        .collect::<Vec<_>>();

    embeds[1] = CreateEmbed::new().title("Tasks").description(tasks);

    ctx.channel_id()
        .edit_message(
            ctx.http(),
            message.id,
            EditMessage::new().add_embeds(embeds),
        )
        .await?;
    Ok(())
}
