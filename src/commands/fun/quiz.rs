use crate::types::{Context, Error};

mod buttons;
mod flags;

/// Play a quiz!
#[poise::command(
    slash_command,
    name_localized("es-ES", "trivia"),
    description_localized("es-ES", "Juega a una trivia!"),
    subcommands("super::quiz::flags::flags", "super::quiz::buttons::buttons"),
    category = "Games"
)]
pub async fn quiz(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}
