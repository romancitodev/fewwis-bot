use std::time::Duration;

use crate::{
    api::{Memes, MEME_API},
    Context, Error,
};
use ::serenity::{
    builder::{CreateActionRow, CreateButton, CreateEmbed, CreateEmbedFooter},
    model::Color,
};
use poise::{serenity_prelude as serenity, CreateReply};

/// Get a meme
#[poise::command(
    slash_command,
    name_localized("es-ES", "memes"),
    description_localized("es-ES", "Obten un meme de forma aleatoria!"),
    category = "Utilities"
)]
pub async fn meme(
    ctx: Context<'_>,
    #[description = "Subreddit to fetch meme"]
    #[name_localized("es-ES", "subreddit")]
    #[description_localized("es-ES", "Subreddit a fetchear")]
    subreddit: Option<String>,
) -> Result<(), Error> {
    let subreddit = subreddit.unwrap_or("ShitpostingLatam".to_owned());

    let reply = CreateReply::new();

    let memes_count = 50;
    let embeds = get_memes(subreddit, memes_count)
        .await?
        .memes
        .iter()
        .map(|meme| {
            CreateEmbed::new()
                .title(format!("😂 Meme of r/{}", meme.subreddit))
                .description(format!("**{}**", meme.title))
                .url(meme.post_link.clone())
                .footer(CreateEmbedFooter::new(format!(
                    "{} upvotes - By u/{}",
                    meme.ups, meme.author
                )))
                .image(meme.url.clone())
                .color(Color::BLITZ_BLUE)
        })
        .collect::<Vec<_>>();

    let mut counter = 0;
    let left = CreateButton::new("left")
        .style(serenity::ButtonStyle::Primary)
        .label("◀")
        .disabled(true);
    let center = CreateButton::new("center")
        .label(format!("{}/{}", counter + 1, embeds.len()))
        .disabled(true)
        .style(serenity::ButtonStyle::Secondary);
    let right = CreateButton::new("right")
        .style(serenity::ButtonStyle::Primary)
        .label("▶");
    let to_beggining = CreateButton::new("beggining")
        .style(serenity::ButtonStyle::Primary)
        .label("⏪")
        .disabled(true);
    let to_final = CreateButton::new("final")
        .style(serenity::ButtonStyle::Primary)
        .label("⏩");

    let buttons = CreateActionRow::Buttons(vec![to_beggining, left, center, right, to_final]);

    let bot_interaction = ctx
        .send(
            reply
                .clone()
                .embed(embeds[0].clone())
                .components(vec![buttons]),
        )
        .await?;

    let message_id = bot_interaction.message().await?.id;

    while let Some(interaction) = bot_interaction
        .message()
        .await?
        .await_component_interactions(ctx.discord().shard.clone())
        .message_id(message_id)
        .timeout(Duration::from_secs(60))
        .await
    {
        match interaction.data.custom_id.as_str() {
            "left" => counter = 0.max(counter - 1),
            "right" => counter = (embeds.len() - 1).min(counter + 1),
            "beggining" => counter = 0,
            "final" => counter = embeds.len() - 1,
            _ => (),
        };

        let left = CreateButton::new("left")
            .style(serenity::ButtonStyle::Primary)
            .label("◀")
            .disabled(counter == 0);
        let center = CreateButton::new("center")
            .label(format!("{}/{}", counter + 1, embeds.len()))
            .disabled(true)
            .style(serenity::ButtonStyle::Secondary);
        let right = CreateButton::new("right")
            .style(serenity::ButtonStyle::Primary)
            .disabled(counter >= embeds.len() - 1)
            .label("▶");
        let to_beggining = CreateButton::new("beggining")
            .style(serenity::ButtonStyle::Primary)
            .label("⏪")
            .disabled(counter == 0);
        let to_final = CreateButton::new("final")
            .style(serenity::ButtonStyle::Primary)
            .label("⏩")
            .disabled(counter >= embeds.len() - 1);

        let buttons = CreateActionRow::Buttons(vec![to_beggining, left, center, right, to_final]);

        interaction
            .create_response(ctx, serenity::CreateInteractionResponse::Acknowledge)
            .await?;
        bot_interaction
            .edit(
                ctx,
                reply
                    .clone()
                    .embed(embeds[counter].clone())
                    .components(vec![buttons]),
            )
            .await?;
    }

    bot_interaction
        .edit(
            ctx,
            reply
                .clone()
                .embed(embeds[counter].clone())
                .components(vec![]),
        )
        .await?;

    Ok(())
}

async fn get_memes(subreddit: String, count: i64) -> Result<Memes, Error> {
    let data = reqwest::get(
        MEME_API
            .replace("{subreddit}", &subreddit)
            .replace("{count}", &count.to_string()),
    )
    .await?
    .json::<Memes>()
    .await?;
    Ok(data)
}
