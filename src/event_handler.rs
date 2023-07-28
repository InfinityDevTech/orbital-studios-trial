use mongodb::bson::doc;
use mongodb::{Collection, Database};
use serenity::async_trait;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::prelude::command::Command;
use serenity::model::prelude::{Activity, Member};
use serenity::model::user::OnlineStatus;
use serenity::prelude::*;

use crate::commands;
use crate::database::User;
use crate::log::{log_debug, log_error, log_info, log_warning};

pub struct Handler {
    pub db: Database
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        log_info(&format!(
            "Bot is connected to Discord as - {}#{}",
            ready.user.name, ready.user.discriminator
        ));
        ctx.set_presence(Some(Activity::watching("you.")), OnlineStatus::Online).await;

        let commands = Command::create_global_application_command(&ctx.http, |command| {commands::info::register(command)}).await;
        let _ = Command::create_global_application_command(&ctx.http, |command| {commands::random::register(command)}).await;
        let _ = Command::create_global_application_command(&ctx.http, |command| {commands::cat::register(command)}).await;
        match commands {
            Ok(_) => log_info("Successfully registered commands!"),
            Err(e) => log_error(&e.to_string()),
        }
    }
    // add to Users DB if they dont exist already.
    async fn guild_member_addition(&self, _ctx: Context, member: Member) {
        let t: Collection<User> = self.db.collection("Users");
        let d = t
            .find_one_and_update(
                doc! {"user_id": member.user.id.as_u64().to_string()},
                doc! {"$inc": {"known_servers": 1}},
                None,
            )
            .await
            .unwrap();
        match d {
            Some(_) => {}
            None => {
                t.insert_one(
                    User {
                        user_id: member.user.id.as_u64().to_string(),
                        commands: 1,
                        known_servers: 1,
                    },
                    None,
                )
                .await
                .unwrap();
            }
        }
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
            let t: Collection<User> = self.db.collection("Users");
            if let Interaction::ApplicationCommand(command) = interaction {
                let d = t
                    .find_one_and_update(
                        doc! {"user_id": command.user.id.to_string()},
                        doc! {"$inc": {"commands": 1}},
                        None,
                    )
                    .await
                    .unwrap();
                match d {
                    Some(_) => {}
                    None => {
                        t.insert_one(
                            User {
                                user_id: command.user.id.as_u64().to_string(),
                                commands: 1,
                                known_servers: 1,
                            },
                            None,
                        )
                        .await
                        .unwrap();
                    }
                }
                log_debug(&format!(
                    "User: {} is using command {}",
                    command.user.name, command.data.name
                ));

                let _response_t = match command.data.name.as_str() {
                    "info" => commands::info::exec(command, ctx, &self.db).await,
                    "random" => commands::random::exec(command, ctx, &self.db).await,
                    "cat" => commands::cat::exec(command, ctx, &self.db).await,
                    _ => {
                        log_warning("Discord sent us an unknown command!");
                    }
                };
            }
    }
}
