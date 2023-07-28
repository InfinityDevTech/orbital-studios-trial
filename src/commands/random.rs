use mongodb::Database;
use rand::Rng;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        application_command::ApplicationCommandInteraction, command::CommandOptionType,
    },
    prelude::Context,
    utils::Colour,
};

pub async fn exec(command: ApplicationCommandInteraction, ctx: Context, _db: &Database) {
    let options = &command.data.options;
    let mut min: i64 = 0;
    let mut max: i64 = 6;
    options.iter().for_each(|o| {
        if (o.name == "min") {
            min = o.value.as_ref().unwrap().as_i64().unwrap();
        } else if (o.name == "max") {
            max = o.value.as_ref().unwrap().as_i64().unwrap();
        }
    });

    let random = rand::thread_rng().gen_range(min..max);

    let _ = command.create_interaction_response(ctx.http, |m| {
        m.kind(serenity::model::prelude::InteractionResponseType::ChannelMessageWithSource).interaction_response_data(|f| {
        f.embed(|e| {
           let embed = e.author(|a| a.name("Random number").icon_url("https://cdn.discordapp.com/icons/805298672475701249/a_d2c6518167f7a6fe61a45aa20179f843.webp"))
                 .fields(vec![
                     ("Random number", random, false),

                 ])
                 .colour(Colour::from_rgb(240,25,184));
             return embed;
         })
        })
    }).await;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("random")
        .description("Get a random number!")
        .create_option(|option| {
            option
                .name("min")
                .description("Minimum number")
                .kind(CommandOptionType::Integer)
                .required(false)
        })
        .create_option(|option| {
            option
                .name("max")
                .description("Maximum number")
                .kind(CommandOptionType::Integer)
                .required(false)
        })
}
