use std::fmt::Write;

use crate::{
    helper::db::{add_steps, get_all_steps, get_post, save_post},
    types::{ApplicationContext, Error},
};
use poise::{serenity_prelude as serenity, Modal};

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
    let post = match get_post(db, channel_id).await {
        Ok(model) => model,
        Err(_) => {
            save_post(
                db,
                channel_id,
                ctx.channel_id()
                    .to_channel(ctx.serenity_context())
                    .await?
                    .to_string(),
            )
            .await?
        }
    };

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
    ctx.say(tasks).await?;
    Ok(())
}
