use mongodb::Database;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::application_command::ApplicationCommandInteraction,
    prelude::Context,
};

pub async fn exec(command: ApplicationCommandInteraction, ctx: Context, _db: &Database) {
    let _ = command.create_interaction_response(ctx.http, |m| {
        m.kind(serenity::model::prelude::InteractionResponseType::ChannelMessageWithSource).interaction_response_data(|d| {
            d.content("Pong!")
        })
    }).await;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("Ping the bot!")
}
