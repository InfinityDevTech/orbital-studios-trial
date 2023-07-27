use config::Config;
use log::log_error;
use mongodb::Database;

use serenity::prelude::*;

mod database;
mod config;
mod log;
mod event_handler;
mod commands;

// Default's to info.
static mut LOG_LEVEL: u8 = 3;

#[tokio::main]
async fn main() {
    let config: Config = config::check_conf();
    unsafe { LOG_LEVEL = config.log_level }
    let db: Database = database::connect(&config.database_uri).await;

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MEMBERS | GatewayIntents::GUILD_BANS | GatewayIntents::GUILD_PRESENCES;
    let mut client = Client::builder(config.token.clone(), intents).event_handler(event_handler::Handler{db}).await.expect("Failed to init client");


    if let Err(e) = client.start().await {
        log_error("Failed to start Discord bot client!");
        log_error(&e.to_string());
        std::process::exit(0)
    }
}
