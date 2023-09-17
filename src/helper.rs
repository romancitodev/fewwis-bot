use poise::{serenity_prelude::Color, CreateReply};
use serenity::builder::CreateEmbed;
use tracing::error;

use crate::types::{Data, Error};

pub enum Colors {
    White,
    Gray,
    Green,
    Orange,
    Red,
    Custom(u8, u8, u8),
}

impl From<Colors> for Color {
    fn from(value: Colors) -> Self {
        match value {
            Colors::White => Color::from_rgb(255, 255, 255),
            Colors::Gray => Color::from_rgb(175, 175, 175),
            Colors::Green => Color::from_rgb(178, 247, 117),
            Colors::Orange => Color::from_rgb(247, 194, 131),
            Colors::Red => Color::from_rgb(247, 131, 131),
            Colors::Custom(r, g, b) => Color::from_rgb(r, g, b),
        }
    }
}

pub async fn handle_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Command { error, ctx } => {
            let reply = CreateReply::new();
            let embed = CreateEmbed::new();
            ctx.send(
                reply
                    .embed(
                        embed
                            .title("❌ Oops... an error ocurred.")
                            .description(error.to_string())
                            .color(serenity::model::Color::RED),
                    )
                    .ephemeral(true),
            )
            .await
            .unwrap();
        }
        poise::FrameworkError::CommandCheckFailed { error, ctx } => {
            let reply = CreateReply::new();
            let embed = CreateEmbed::new();
            ctx.send(
                reply
                    .embed(
                        embed
                            .title("❌ Oops... an error ocurred.")
                            .description(error.unwrap_or("Unexpected".into()).to_string())
                            .color(serenity::model::Color::RED),
                    )
                    .ephemeral(true),
            )
            .await
            .unwrap();
        }
        poise::FrameworkError::Setup { error, .. } => {
            error!("❌ Errot setting up the bot. {error:?}");
        }
        err => {
            if let Err(err) = poise::builtins::on_error(err).await {
                error!("❌ Error handling {err}");
            }
        }
    };
}
