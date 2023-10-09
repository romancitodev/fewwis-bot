use std::fmt::Write;

use crate::{
    helper::{
        db::{get_all_steps, get_post, update_task},
        Colors,
    },
    types::{Context, Error},
};
use poise::Modal;
use serenity::builder::{CreateEmbed, EditMessage};

#[derive(Debug, Modal)]
#[name = "To-do List"]
struct TaskModal {
    #[name = "Milestone"]
    #[placeholder = "Fill this with your milestone"]
    #[min_length = 1]
    #[paragraph]
    task_list: String,
}

/// Update the state of a step.
#[poise::command(
    slash_command,
    name_localized("es-ES", "actualizar"),
    description_localized("es-ES", "Actualiza el estado de un paso"),
    category = "Utilities"
)]
pub async fn update(
    ctx: Context<'_>,
    #[description = "The task to update"]
    #[name_localized("es-ES", "tarea")]
    #[description_localized("es-ES", "La tarea a actualizar")]
    #[autocomplete = "super::task_autocompleter"]
    task: i32,
    #[description = "finished"]
    #[name_localized("es-ES", "finalizada")]
    #[description_localized("es-ES", "El estado de la tarea")]
    finished: bool,
) -> Result<(), Error> {
    let db = &ctx.data().db;

    let post = get_post(db, ctx.channel_id().get()).await?;
    update_task(db, task, finished).await?;

    let tasks = get_all_steps(db, post.id)
        .await?
        .iter()
        .fold(String::new(), |mut acc, task| {
            writeln!(
                &mut acc,
                "> {} **{}.** {}",
                if task.completed != 0 { "✅" } else { "⏳" },
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

    embeds[1] = CreateEmbed::new()
        .title("Tasks")
        .description(tasks)
        .color(Colors::Fewwis);

    ctx.channel_id()
        .edit_message(
            ctx.http(),
            message.id,
            EditMessage::new().add_embeds(embeds),
        )
        .await?;

    ctx.defer_ephemeral().await?;
    ctx.say("Task updated!").await?;

    Ok(())
}
