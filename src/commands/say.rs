use mongodb::Database;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::application_command::ApplicationCommandInteraction,
    prelude::Context,
};

pub async fn exec(command: ApplicationCommandInteraction, ctx: Context, _db: &Database) {
    let say = command.data.options.first().unwrap().value.as_ref().unwrap().as_str().unwrap();
    let _ = command.create_interaction_response(ctx.http, |m| {
        m.kind(serenity::model::prelude::InteractionResponseType::ChannelMessageWithSource).interaction_response_data(|d| {
            d.content(say)
        })
    }).await;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("say").description("Make the bot say something!").create_option(|o| o.kind(serenity::model::prelude::command::CommandOptionType::String).name("message").required(true))
}
