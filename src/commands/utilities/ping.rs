use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Pong!
#[poise::command(slash_command, category = "Utilities")]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let ping = (ctx.created_at().timestamp_millis() - serenity::Timestamp::now().timestamp_millis())
        as f32
        / 100.0;

    let color = match ping as i32 {
        i32::MIN..0 => (175, 175, 175),
        0..100 => (145, 247, 131),
        100..200 => (247, 235, 131),
        _ => (247, 153, 131),
    };

    ctx.send(|msg| {
        msg.embed(|embed| {
            embed
                .title("🏓 Pong")
                .description(format!("📡 {ping}ms"))
                .color(color)
        })
        .allowed_mentions(|m| m.empty_parse())
    })
    .await?;

    Ok(())
}
