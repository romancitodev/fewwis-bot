use crate::{Context, Error};
use ::serenity::{builder::CreateEmbed, model::Color};
use poise::{serenity_prelude as serenity, CreateReply};
use reqwest::header;
use serde::Deserialize;

/// Get the avatar of any member in the server
#[poise::command(context_menu_command = "Translate text", category = "Utilities")]
pub async fn translate_ctx_menu(
    ctx: Context<'_>,
    #[description = "The member to fetch"] msg: serenity::Message,
) -> Result<(), Error> {
    let text = msg.content;
    let response = get_translation(text.clone()).await?;

    let reply = CreateReply::new();
    let embed = CreateEmbed::new();

    ctx.send(
        reply.embed(
            embed
                .title("📄 Translator")
                .description(format!(
                    "> 📤 **Original Text**: `{}`\n\n> ✨ **Translated Text**: `{}`",
                    text, response.data.translated_text
                ))
                .color(Color::BLURPLE),
        ),
    )
    .await?;
    Ok(())
}

/// Translate a text
#[poise::command(slash_command, category = "Utilities")]
pub async fn translate(
    ctx: Context<'_>,
    #[description = "The member to fetch"] text: String,
) -> Result<(), Error> {
    let response = get_translation(text.clone()).await?;

    let reply = CreateReply::new();
    let embed = CreateEmbed::new();

    ctx.send(
        reply.embed(
            embed
                .title("📄 Translator")
                .description(format!(
                    "> 📤 **Original Text**: `{}`\n> ✨ **Translated Text**: `{}`",
                    text, response.data.translated_text
                ))
                .color(Color::BLURPLE),
        ),
    )
    .await?;
    Ok(())
}

async fn get_translation(text: String) -> Result<Translator, Error> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "content-type",
        "application/x-www-form-urlencoded".parse().unwrap(),
    );
    headers.insert(
        "X-RapidAPI-Key",
        dotenvy::var("RAPID_API_KEY").unwrap().parse().unwrap(),
    );
    headers.insert(
        "X-RapidAPI-Host",
        "text-translator2.p.rapidapi.com".parse().unwrap(),
    );

    let client = reqwest::Client::new();
    Ok(client
        .post("https://text-translator2.p.rapidapi.com/translate")
        .headers(headers)
        .form(&[
            ("source_language", "en"),
            ("target_language", "es"),
            ("text", &text.clone()),
        ])
        .send()
        .await?
        .json::<Translator>()
        .await?)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Translator {
    data: TranslatorData,
    status: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TranslatorData {
    translated_text: String,
}
