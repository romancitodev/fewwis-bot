use crate::types::{Context, Error};

mod add;

/// Set the milestone of a task
#[poise::command(
    slash_command,
    name_localized("es-ES", "pasos"),
    description_localized("es-ES", "Establece los pasos a seguir dentro de la tarea"),
    category = "Utilities",
    subcommands("add::add")
)]
pub async fn steps(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}
