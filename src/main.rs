#![feature(exclusive_range_pattern)]

use std::collections::HashSet;

use ::serenity::gateway::ActivityData;
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
    poise::FrameworkBuilder::default()
        .token(dotenvy::var("BOT_TOKEN").expect("❌ Missing BOT_TOKEN in .env file"))
        .intents(serenity::GatewayIntents::all())
        .options(poise::FrameworkOptions {
            commands: commands::all(),
            owners: HashSet::from([401845716991082496.into()]),
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
        .user_data_setup(move |ctx, _ready, fm| {
            Box::pin(async move {
                info!("👷 Setting up the bot...");
                ctx.set_activity(Some(ActivityData::playing("Casio Theme 4 life")));
                info!("🕹 Setted activity.");
                let commands = &fm.options().commands;
                let create_commands = poise::builtins::create_application_commands(commands);

                serenity::Command::set_global_commands(ctx, create_commands).await?;
                info!("📤 Registered commands.");
                Ok(Data {})
            })
        })
        .run()
        .await
        .unwrap();
}
