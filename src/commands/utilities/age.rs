use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Get the account age of any user
#[poise::command(
    slash_command,
    name_localized("es-ES", "edad"),
    description_localized("es-ES", "Obten la edad de la cuenta de cualquier usuario!"),
    rename = "age",
    category = "Utilities"
)]
pub async fn get_age(
    ctx: Context<'_>,
    #[description = "Selected user"]
    #[name_localized("es-ES", "usuario")]
    #[description_localized("es-ES", "Usuario a escanear")]
    user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}
