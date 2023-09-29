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
    let user_opt = command
        .data
        .options
        .first()
        .unwrap()
        .resolved
        .as_ref()
        .unwrap();

    if let CommandDataOptionValue::User(user, guildmember) = user_opt {
        if let Some(member) = guildmember {
            let guild_member = ctx.http.get_member(member.guild_id.unwrap().into(), member.user.as_ref().unwrap().id.into()).await;
            let _ = guild_member.unwrap().ban(&ctx, 7).await;
            let _ = command
                .create_interaction_response(ctx.http, |m| {
                    m.kind(
                        serenity::model::prelude::InteractionResponseType::ChannelMessageWithSource,
                    )
                    .interaction_response_data(|d| d.content(format!("Banned {}", member.user.as_ref().unwrap().name)))
                })
                .await;
        }
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("ban")
        .description("Ban a user")
        .create_option(|o| {
            o.kind(serenity::model::prelude::command::CommandOptionType::User)
                .required(true)
        })
}
