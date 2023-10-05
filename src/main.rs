#![feature(exclusive_range_pattern)]

use ::serenity::{gateway::ActivityData, Client};
use consts::OWNER_BOT;
use helper::handle_error;
use poise::serenity_prelude as serenity;
use sea_orm::{ConnectionTrait, Database, Statement};
use shuttle_poise::ShuttlePoise;
use shuttle_runtime::Context as ShuttleContext;
use shuttle_secrets::SecretStore;
use std::collections::HashSet;
use tracing::{error, info};
mod api;
mod commands;
mod consts;
mod entities;
mod helper;
mod types;
use types::*;

#[shuttle_runtime::main]
async fn main(#[shuttle_secrets::Secrets] store: SecretStore) -> ShuttlePoise<Data, Error> {
    let bot_token = store
        .get("BOT_TOKEN")
        .context("❌ Missing BOT_TOKEN in Secrets.toml file")?;
    let db_url = store
        .get("DATABASE_URL")
        .context("❌ Missing DATABASE_URL in Secrets.toml file")?;
    let db_name = store
        .get("DATABASE_URL")
        .context("❌ Missing DATABASE_NAME in Secrets.toml file")?;
    tracing_subscriber::fmt::init();

    let framework = poise::FrameworkBuilder::default()
        .token(bot_token)
        .intents(serenity::GatewayIntents::all())
        .options(poise::FrameworkOptions {
            commands: commands::all(),
            owners: HashSet::from([OWNER_BOT.into()]),
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
        });
    // .setup(move |ctx, _ready, fm| {
    //     Box::pin(async move {
    //         info!("👷 Setting up the bot...");
    //         ctx.set_activity(Some(ActivityData::playing("Casio Theme 4 life")));
    //         info!("🕹 Setted activity.");
    //         let commands = &fm.options().commands;
    //         let create_commands = poise::builtins::create_application_commands(commands);

    //         info!("🔁 Registering commands...");
    //         serenity::Command::set_global_commands(ctx, create_commands).await?;
    //         info!("📤 Registered commands.");
    //         info!("📡 Connecting to database...");
    //         let connection = Database::connect(db_url.clone()).await?;
    //         connection
    //             .execute(Statement::from_string(
    //                 connection.get_database_backend(),
    //                 format!("CREATE DATABASE IF NOT EXISTS `{}`;", db_name.clone()),
    //             ))
    //             .await?;
    //         let connection = Database::connect(format!("{db_url}{db_name}")).await?;
    //         info!("📡 Connection to database successfull.");
    //         info!("✅ Bot initialized.");
    //         Ok(Data::new(connection))
    //     })
    // });
    let bot = poise::Framework::new(
        poise::FrameworkOptions {
            commands: commands::all(),
            owners: HashSet::from([OWNER_BOT.into()]),
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
        },
        move |ctx, _ready, fm| {
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
        },
    );
    Ok(bot.into())
}
