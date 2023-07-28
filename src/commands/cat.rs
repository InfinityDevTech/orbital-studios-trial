use mongodb::Database;
use serde::Deserialize;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::application_command::ApplicationCommandInteraction,
    prelude::Context,
    utils::Colour,
};

#[derive(Deserialize, Clone)]
struct CatResp {
    url: String,
}

pub async fn exec(command: ApplicationCommandInteraction, ctx: Context, _db: &Database) {
    let cat = reqwest::get("https://api.thecatapi.com/v1/images/search")
        .await
        .unwrap()
        .json::<Vec<CatResp>>()
        .await
        .unwrap();

    let _ = command.create_interaction_response(ctx.http, |m| {
        m.kind(serenity::model::prelude::InteractionResponseType::ChannelMessageWithSource).interaction_response_data(|f| {
        f.embed(|e| {
           let embed = e.author(|a| a.name("Cat").icon_url("https://cdn.discordapp.com/icons/805298672475701249/a_d2c6518167f7a6fe61a45aa20179f843.webp"))
                .image(&cat.get(0).unwrap().url)
                .colour(Colour::from_rgb(240,25,184));
             return embed;
         })
        })
    }).await;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("cat").description("Get a random cat!")
}
