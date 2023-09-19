use poise::{serenity_prelude as serenity, serenity_prelude::Color, CreateReply};
use serenity::builder::{CreateActionRow, CreateButton, CreateEmbed};
use std::time::Duration;
use tracing::error;

use crate::types::{Context, Data, Error};

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

type Function = fn(&Context, &str, i32, i32) -> i32;
type Conditional = fn(usize, i32, i32) -> bool;

pub struct Paginator {
    pages: Vec<CreateEmbed>,
    counter: i32,
    additional_fn: Option<Function>,
    additional_cond: Option<Conditional>,
    additional_row: Option<CreateActionRow>,
}

impl Paginator {
    pub fn new(pages: Vec<CreateEmbed>) -> Paginator {
        Paginator {
            pages,
            counter: 0,
            additional_fn: None,
            additional_cond: None,
            additional_row: None,
        }
    }

    pub fn add_row(self, row: CreateActionRow, r#fn: Function, cond: Conditional) -> Self {
        Self {
            additional_row: Some(row),
            additional_fn: Some(r#fn),
            additional_cond: Some(cond),
            ..self
        }
    }

    pub async fn paginate(&mut self, ctx: Context<'_>) -> Result<(), Error> {
        let reply = CreateReply::new();
        let buttons = self.create_buttons();

        let initial = ctx
            .send(
                reply
                    .clone()
                    .embed(self.pages[self.counter as usize].clone())
                    .components(buttons),
            )
            .await?;

        let message_id = initial.message().await?.id;

        while let Some(interaction) = initial
            .message()
            .await?
            .await_component_interactions(ctx.discord().shard.clone())
            .message_id(message_id)
            .timeout(Duration::from_secs(60))
            .await
        {
            match &*interaction.data.custom_id {
                "left" => {
                    self.counter = 0.max(self.counter - 1);
                }
                "right" => {
                    self.counter = (self.pages.len() as i32 - 1).min(self.counter + 1);
                }
                "beggining" => self.counter = 0,
                "final" => {
                    self.counter = self.pages.len() as i32 - 1;
                }
                "delete" => {
                    initial.delete(ctx).await?;
                    return Ok(());
                }
                id => {
                    if let Some(additional_fn) = &self.additional_fn {
                        self.counter =
                            (additional_fn)(&ctx, id, self.counter, self.pages.len() as i32 - 1);
                    }
                }
            };

            let buttons = self.create_buttons();

            interaction
                .create_response(ctx, serenity::CreateInteractionResponse::Acknowledge)
                .await?;
            initial
                .edit(
                    ctx,
                    reply
                        .clone()
                        .embed(self.pages[self.counter as usize].clone())
                        .components(buttons),
                )
                .await?;
        }

        initial
            .edit(
                ctx,
                reply
                    .clone()
                    .embed(self.pages[self.counter as usize].clone())
                    .components(vec![]),
            )
            .await?;
        Ok(())
    }

    pub(self) fn create_buttons(&self) -> Vec<CreateActionRow> {
        let mut buttons_row = vec![];
        let left = CreateButton::new("left")
            .style(serenity::ButtonStyle::Primary)
            .label("◀")
            .disabled(self.counter == 0);
        let center = CreateButton::new("center")
            .label(format!("{}/{}", self.counter + 1, self.pages.len()))
            .disabled(true)
            .style(serenity::ButtonStyle::Secondary);
        let right = CreateButton::new("right")
            .style(serenity::ButtonStyle::Primary)
            .label("▶")
            .disabled(self.counter >= self.pages.len() as i32 - 1);
        let to_beggining = CreateButton::new("beggining")
            .style(serenity::ButtonStyle::Primary)
            .label("⏪")
            .disabled(self.counter == 0);
        let to_final = CreateButton::new("final")
            .style(serenity::ButtonStyle::Primary)
            .label("⏩")
            .disabled(self.counter >= self.pages.len() as i32 - 1);

        let buttons = CreateActionRow::Buttons(vec![to_beggining, left, center, right, to_final]);

        buttons_row.push(buttons);

        if let Some(CreateActionRow::Buttons(additional_rows)) = &self.additional_row {
            let rows = CreateActionRow::Buttons(
                additional_rows
                    .iter()
                    .cloned()
                    .enumerate()
                    .map({
                        |(index, b)| {
                            let is_disabled = self.additional_cond.unwrap()(
                                index + 1,
                                self.counter,
                                self.pages.len() as i32,
                            );
                            b.disabled(is_disabled)
                        }
                    })
                    .collect::<Vec<_>>(),
            );
            buttons_row.push(rows)
        }

        buttons_row
    }
}
