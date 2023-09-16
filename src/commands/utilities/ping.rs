use crate::{helper::Colors, Context, Error};
use ::serenity::builder::{CreateAllowedMentions, CreateEmbed};
use poise::{serenity_prelude as serenity, CreateReply};

/// Pong!
#[poise::command(slash_command, category = "Utilities")]
pub async fn ping(
    ctx: Context<'_>,
    #[description = "Make the message ephemeral?"] ephemeral: Option<bool>,
) -> Result<(), Error> {
    let ping = (ctx.created_at().timestamp_millis() - serenity::Timestamp::now().timestamp_millis())
        as f32
        / 10.0;

    let color = match ping as i32 {
        i32::MIN..0 => Colors::Gray,
        0..100 => Colors::Green,
        100..200 => Colors::Orange,
        _ => Colors::Red,
    };

    let reply = CreateReply::new();
    let embed = CreateEmbed::new();
    ctx.send(
        reply
            .embed(
                embed
                    .title("🏓 Pong")
                    .description(format!("📡 `{ping}ms`"))
                    .color(color),
            )
            .allowed_mentions(CreateAllowedMentions::new().empty_users())
            .ephemeral(ephemeral.unwrap_or_default()),
    )
    .await?;

    Ok(())
}
