use raelib::prelude::client::RaeClient;
use rand::{self, seq::SliceRandom, Rng};
use sea_orm::EntityTrait;
use std::time::Duration;

use crate::{
    entities::buttons,
    helper::{
        db::{get_buttons, get_user},
        Colors,
    },
    Context, Error,
};
use ::serenity::{
    builder::{CreateEmbed, CreateEmbedFooter},
    model,
};
use poise::{serenity_prelude as serenity, CreateReply};

#[derive(PartialEq, Eq)]
enum Reason {
    Correct,
    Loss,
    Timeout,
}

/// Set the quiz about words
#[poise::command(
    slash_command,
    name_localized("es-ES", "botones"),
    description_localized("es-ES", "Trivia de botones!"),
    subcommands("play_buttons", "stats_buttons"),
    category = "Games"
)]
pub async fn buttons(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Play a quiz of words!
#[poise::command(
    slash_command,
    name_localized("es-ES", "jugar"),
    description_localized("es-ES", "Juega a una trivia de botones!"),
    rename = "play",
    category = "Games"
)]
pub async fn play_buttons(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    let msg = ctx
        .send(
            CreateReply::new().embed(
                CreateEmbed::new()
                    .title("⏳ Starting game...")
                    .color(Colors::White),
            ),
        )
        .await?;
    let reply = CreateReply::new();
    let embed = CreateEmbed::new();
    let db = &ctx.data().db;
    let (user_id, guild_id) = (ctx.author().id, ctx.guild_id().unwrap());
    let (_, stats) = get_user(user_id, guild_id, db).await?;
    let _request_buttons = get_buttons(stats, db).await?;

    let rae_client = RaeClient::default();
    let words = {
        let mut w = rae_client.get_random_with_range(0..4).await?;
        let mut rng = rand::thread_rng();
        w.shuffle(&mut rng);
        w.clone()
    };
    // words.shuffle(&mut rng);

    let _reason = Reason::Timeout;

    // msg.edit(ctx, reply.content("hello!")).await?;
    msg.edit(
        ctx,
        reply.clone().embed(
            embed
                .clone()
                .title("🤓 Quiz of Definitions")
                .color(Colors::Fewwis)
                .field(
                    "☝️ Definition:",
                    format!("> {}", words[0].definitions()[0].definition()),
                    true,
                ),
        ),
    )
    .await?;

    // while let Some(message) = serenity::MessageCollector::new(ctx.serenity_context().shard.clone())
    //     .author_id(ctx.author().id)
    //     .timeout(Duration::from_secs(60))
    //     .await
    // {}

    // let active_buttons: buttons::ActiveModel = request_buttons.into();
    // let updated_flags = match reason {
    //     Reason::Correct => {
    //         msg.edit(
    //             ctx,
    //             reply.clone().embed(get_embed_flag(
    //                 Colors::Green,
    //                 "> 🎉 **Congrats!** You got it right.",
    //                 &flag,
    //                 counter,
    //             )),
    //         )
    //         .await?;
    //         match counter {
    //             0 => flags::ActiveModel {
    //                 first_attempt: sea_orm::ActiveValue::Set(
    //                     active_flags.first_attempt.unwrap() + 1,
    //                 ),
    //                 ..active_flags
    //             },
    //             1 => flags::ActiveModel {
    //                 second_attempt: sea_orm::ActiveValue::Set(
    //                     active_flags.second_attempt.unwrap() + 1,
    //                 ),
    //                 ..active_flags
    //             },
    //             2 => flags::ActiveModel {
    //                 third_attempt: sea_orm::ActiveValue::Set(
    //                     active_flags.third_attempt.unwrap() + 1,
    //                 ),
    //                 ..active_flags
    //             },
    //             _ => unreachable!(),
    //         }
    //     }
    //     reason @ Reason::Loss | reason @ Reason::Timeout => {
    //         if Reason::Timeout == reason {
    //             msg.edit(
    //                 ctx,
    //                 reply.clone().embed(get_embed_flag(
    //                     Colors::Red,
    //                     "> ⌛ **Timeout**! you didn't guess in time.",
    //                     &flag,
    //                     counter,
    //                 )),
    //             )
    //             .await?;
    //         };
    //         flags::ActiveModel {
    //             wrong: sea_orm::ActiveValue::Set(active_flags.wrong.unwrap() + 1),
    //             ..active_flags
    //         }
    //     }
    // };

    // flags::Entity::update(updated_flags).exec(db).await?;

    Ok(())
}

/// View your stats in quiz flags
#[poise::command(
    slash_command,
    name_localized("es-ES", "estadisticas"),
    description_localized("es-ES", "Mira tus estadisticas sobre la trivia de botones!"),
    rename = "stats",
    category = "Games"
)]
pub async fn stats_buttons(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    let msg = ctx
        .send(
            CreateReply::new().embed(
                CreateEmbed::new()
                    .title("⏳ Fetching data...")
                    .color(Colors::White),
            ),
        )
        .await?;
    let db = &ctx.data().db;
    let (user_id, guild_id) = (ctx.author().id, ctx.guild_id().unwrap());
    let (_, stats) = get_user(user_id, guild_id, db).await?;
    let buttons = get_buttons(stats, db).await?;

    let total =
        buttons.first_attempt + buttons.second_attempt + buttons.third_attempt + buttons.wrong;

    let first_attempt_percentage = if total == 0 {
        0.0
    } else {
        (buttons.first_attempt as f64 / total as f64) * 100.0
    };
    let second_attempt_percentage = if total == 0 {
        0.0
    } else {
        (buttons.second_attempt as f64 / total as f64) * 100.0
    };
    let third_attempt_percentage = if total == 0 {
        0.0
    } else {
        (buttons.third_attempt as f64 / total as f64) * 100.0
    };
    let wrong_percentage = if total == 0 {
        0.0
    } else {
        (buttons.wrong as f64 / total as f64) * 100.0
    };

    msg.edit(
        ctx,
        CreateReply::new().embed(
            CreateEmbed::new()
                .title(format!("🏁 Quiz stats of {}", ctx.author().name))
                .color(model::Color::BLURPLE)
                .fields([
                    (
                        "1️⃣ Attempt:",
                        format!(
                            "{} ({:.2}%)",
                            buttons.first_attempt, first_attempt_percentage
                        ),
                        true,
                    ),
                    (
                        "2️⃣ Attempt:",
                        format!(
                            "{} ({:.2}%)",
                            buttons.second_attempt, second_attempt_percentage
                        ),
                        true,
                    ),
                    (
                        "3️⃣ Attempt:",
                        format!(
                            "{} ({:.2}%)",
                            buttons.third_attempt, third_attempt_percentage
                        ),
                        true,
                    ),
                    (
                        "❌ Wrong:",
                        format!("{} ({:.2}%)", buttons.wrong, wrong_percentage),
                        true,
                    ),
                    ("🔢 Total:", format!("{}", total), true),
                ]),
        ),
    )
    .await?;

    Ok(())
}
