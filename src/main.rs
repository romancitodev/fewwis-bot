#![feature(exclusive_range_pattern)]

use std::collections::HashSet;

use ::serenity::gateway::ActivityData;
use helper::handle_error;
use poise::serenity_prelude as serenity;
use sea_orm::{ConnectionTrait, Database, Statement};
use tracing::{error, info};
mod api;
mod commands;
mod entities;
mod helper;
mod types;

use types::*;

#[tokio::main]
async fn main() {
    if let Err(e) = dotenvy::dotenv() {
        error!("❌ `dotenv`: {e}");
        std::process::exit(0);
    };
    tracing_subscriber::fmt::init();
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
            on_error: |error| Box::pin(handle_error(error)),
            ..Default::default()
        })
        .setup(move |ctx, _ready, fm| {
            Box::pin(async move {
                info!("👷 Setting up the bot...");
                ctx.set_activity(Some(ActivityData::playing("Casio Theme 4 life")));
                info!("🕹 Setted activity.");
                let commands = &fm.options().commands;
                let create_commands = poise::builtins::create_application_commands(commands);

                info!("🔁 Registering commands...");
                serenity::Command::set_global_commands(ctx, create_commands).await?;
                info!("📤 Registered commands.");
                info!("📡 Connecting to database...");
                let (db_url, db_name) = (
                    dotenvy::var("DATABASE_URL")?,
                    dotenvy::var("DATABASE_NAME")?,
                );
                let connection = Database::connect(db_url.clone()).await?;
                connection
                    .execute(Statement::from_string(
                        connection.get_database_backend(),
                        format!("CREATE DATABASE IF NOT EXISTS `{}`;", db_name.clone()),
                    ))
                    .await?;
                let connection = Database::connect(format!("{db_url}{db_name}")).await?;
                info!("📡 Connection to database successfull.");
                info!("✅ Bot initialized.");
                Ok(Data::new(connection))
            })
        })
        .run()
        .await
        .unwrap();
}
