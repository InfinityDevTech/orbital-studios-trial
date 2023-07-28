use std::collections::HashMap;

use mongodb::{Database, bson::doc};
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
        command::CommandOptionType,
        ChannelId, ChannelType, GuildChannel,
    },
    prelude::Context,
    utils::Colour,
};
use time::format_description;

use crate::{log::log_warning, database::User};

pub async fn exec(command: ApplicationCommandInteraction, ctx: Context, db: &Database) {
    let options = &command.data.options;
    let option = options.first().unwrap();

    match option.name.as_str() {
        "user" => {
            let user_opt = option.options.first().unwrap().resolved.as_ref().unwrap();
            let mut temp: Vec<String> = Vec::new();
            if let CommandDataOptionValue::User(user, guildmember) = user_opt {
                let roles = &guildmember.as_ref().unwrap().roles;
                roles.iter().for_each(|r| {
                    temp.push(format!("<@&{}>", r.0))
                });
                let t = temp.clone();
                let user_db: User = db.collection("Users").find_one(doc!{"user_id": command.user.id.to_string()}, None).await.unwrap().unwrap();
                command.create_interaction_response(ctx.http, |m| {
                   m.kind(serenity::model::prelude::InteractionResponseType::ChannelMessageWithSource).interaction_response_data(|f| {
                   f.embed(|e| {
                      let embed = e.author(|a| a.name("User info").icon_url("https://cdn.discordapp.com/icons/805298672475701249/a_d2c6518167f7a6fe61a45aa20179f843.webp"))
                            .fields(vec![
                                ("Username", &user.name, false),
                                ("Joined", &guildmember.as_ref().unwrap().joined_at.unwrap().format(&format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap()).unwrap().to_string(), true),
                                ("Registered", &command.user.created_at().format(&format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap()).unwrap().to_string(), true),
                                ("\u{200b}", &"\u{200b}".to_string(), false),
                                ("Commands run", &user_db.commands.to_string(), true),
                                ("Known servers", &user_db.known_servers.to_string(), true),
                                (&format!("Roles [{}]", roles.len()), &t.join(" "), false)

                            ])
                            .colour(Colour::from_rgb(240,25,184))
                            .footer(|f| f.text(format!("ID - {}", &user.id.to_string())));
                        if user.avatar_url().is_some() {
                            embed.thumbnail(user.avatar_url().unwrap());
                        }
                        return embed;
                    })
                    })
                }).await.unwrap();
            }
        }
        "server" => {
            let guild = ctx
                .http
                .get_guild(command.guild_id.unwrap().0)
                .await
                .unwrap();
            let channels = guild.channels(&ctx.http).await.unwrap();
            //let members = &guild.members(&ctx.http, None, None).await.unwrap();
            command
                .create_interaction_response(ctx.http, |m| {
                    m.kind(serenity::model::prelude::InteractionResponseType::ChannelMessageWithSource,)
                    .interaction_response_data(|f| {
                        f.embed(|e| {
                            e.author(|a| a.name("Server info").icon_url("https://cdn.discordapp.com/icons/805298672475701249/a_d2c6518167f7a6fe61a45aa20179f843.webp"))
                                .thumbnail(guild.icon_url().unwrap())
                                .fields(vec![
                                    (format!("Name - {}", &guild.name), "\u{200b}", false),
                                    (format!("Server ID - {}", &guild.id.to_string()),"\u{200b}",false,),
                                    (format!("Text channels - {}", channels.iter().filter(|c| c.1.kind == ChannelType::Text || c.1.kind == ChannelType::News).collect::<HashMap<&ChannelId, &GuildChannel>>().len()), "\u{200b}", false),
                                    (format!("Voice channels - {}", channels.iter().filter(|c| c.1.kind == ChannelType::Voice || c.1.kind == ChannelType::Stage).collect::<HashMap<&ChannelId, &GuildChannel>>().len()), "\u{200b}", false),
                                    (format!("Total channels - {}", &channels.iter().filter(|c| c.1.kind != ChannelType::Category).collect::<HashMap<&ChannelId, &GuildChannel>>().len()), "\u{200b}",false,)
                                    //(format!("Member count   - {}", {members.iter().filter(|u| u.user.bot == false).cloned().}), "\u{200b}", false),
                                ])
                                .colour(Colour::from_rgb(240, 25, 184))
                        })
                    })
                })
                .await
                .unwrap();
        }
        "bot" => {
            command.create_interaction_response(ctx.http, |m| {
                m.kind(serenity::model::prelude::InteractionResponseType::ChannelMessageWithSource).interaction_response_data(|f| {
                f.embed(|e| {
                   let embed = e.author(|a| a.name("Bot info").icon_url("https://cdn.discordapp.com/icons/805298672475701249/a_d2c6518167f7a6fe61a45aa20179f843.webp"))
                         .fields(vec![
                             ("Version", "1.0", true),
                             ("Library", "Eris", true),
                             ("Creator", "inf5", true),
                         ])
                         .colour(Colour::from_rgb(240,25,184));
                     return embed;
                 })
                 })
             }).await.unwrap();
        }
        _ => {
            log_warning("Discord sent the info command an unknown subcommand");
        }
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("info")
        .description("Get info about a user!")
        .create_option(|option| {
            option
                .name("server")
                .description("Get this servers information")
                .kind(CommandOptionType::SubCommand)
        })
        .create_option(|option| {
            option
                .name("user")
                .description("Get information about a user")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|o| {
                    o.name("user")
                        .description("The user who we should get information for")
                        .kind(CommandOptionType::User)
                        .required(true)
                })
        })
        .create_option(|option| {
            option
                .name("bot")
                .description("Get information about the bot")
                .kind(CommandOptionType::SubCommand)
        })
}
