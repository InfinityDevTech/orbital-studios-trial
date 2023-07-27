use mongodb::Database;
use serenity::model::prelude::Activity;
use serenity::model::prelude::command::Command;
use serenity::model::user::OnlineStatus;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::model::application::interaction::Interaction;

use crate::commands;
use crate::log::{log_info, log_error, log_debug, log_warning};

pub struct Handler {
    pub db: Database
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        log_info(&format!("Bot is connected to Discord as - {}#{}", ready.user.name, ready.user.discriminator));
        ctx.set_presence(Some(Activity::watching("you.")), OnlineStatus::Online).await;

        let commands = Command::create_global_application_command(&ctx.http, |command| {
            commands::info::register(command)
        }).await;
        match commands {
            Ok(_) => log_info("Successfully registered commands!"),
            Err(e) => log_error(&e.to_string())
        }
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            log_debug(&format!("User: {} is using command {}", command.user.name, command.data.name));

            let _response_t = match command.data.name.as_str() {
                "info" => commands::info::exec(command, ctx).await,
                _ => {
                    log_warning("Discord sent us an unknown command!");
                }
            };
        }
    }
}