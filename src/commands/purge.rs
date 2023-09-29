use mongodb::Database;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
        PartialMember,
    },
    prelude::Context,
};

pub async fn exec(command: ApplicationCommandInteraction, ctx: Context, _db: &Database) {
    let channel_opt = command
        .data
        .options
        .first()
        .unwrap()
        .resolved
        .as_ref()
        .unwrap();

    if let CommandDataOptionValue::Channel(channel) = channel_opt {

    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("purge")
        .description("Purge messages from a channel")
        .create_option(|o| {
            o.kind(serenity::model::prelude::command::CommandOptionType::Channel)
                .required(true)
        })
}
