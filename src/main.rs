#![feature(exclusive_range_pattern)]

use std::collections::HashSet;

use poise::serenity_prelude as serenity;
use tracing::{error, info};
mod commands;
mod helper;
mod types;

use types::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    if let Err(e) = dotenvy::dotenv() {
        error!("❌ `dotenv`: {e}");
        std::process::exit(0);
    };
    let bot = poise::Framework::builder()
        .token(dotenvy::var("BOT_TOKEN").expect("❌ Missing BOT_TOKEN in .env file"))
        .intents(serenity::GatewayIntents::all())
        .options(poise::FrameworkOptions {
            skip_checks_for_owners: true,
            commands: commands::all(),
            owners: HashSet::from([serenity::UserId(401845716991082496)]),
            pre_command: |ctx| {
                Box::pin(async move {
                    info!(
                        "📥 Starting interaction (command {})",
                        &ctx.invoked_command_name()
                    );
                })
            },
            post_command: |ctx| {
                Box::pin(
                    async move { info!("✅ Command executed ({})", &ctx.invoked_command_name()) },
                )
            },
            ..Default::default()
        })
        .setup(move |ctx, _ready, fm| {
            Box::pin(async move {
                info!("👷 Setting up the bot...");
                ctx.set_activity(serenity::Activity::playing("Casio Theme 4 life"))
                    .await;
                info!("🕹 Setted activity.");
                poise::builtins::register_globally(ctx, &fm.options().commands).await?;
                info!("📤 Registered commands.");
                Ok(Data {})
            })
        })
        .build()
        .await
        .unwrap();

    bot.start().await.unwrap();
}
