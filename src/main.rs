#![allow(non_snake_case)]
mod command;
mod configs;
mod extentions;

use extentions::*;
use serenity::{
    async_trait,
    model::{
        application::{command::CommandOptionType, interaction::Interaction},
        gateway::Ready,
        prelude::{command::Command, GuildId},
    },
    prelude::*,
};

// use std::{
//     thread,
//     time::{Duration, Instant},
// };

// use crate::extentions::conversions::currency;

const AUTHOR: &str = "Decator";
const GITHUB: &str = "https://github.com/El-Wumbus/Bot-of-Justice";
const VERSION: &str = "0.2.3";

// const TWELVE_HOURS: u64 = 43_200;

// pub async fn pull_api()
// {
//     let keys =configs::CONFIG.keys.clone();
//     conversions::currency::EchangeRates::from_api(keys.exchange_rate_api_key).await;
// }

#[tokio::main]
async fn main()
{
    println!(
        "Starting BOJ (Bot of Justice) Version {}.\nWritten by {}. See the source code at '{}'",
        VERSION, AUTHOR, GITHUB
    );

    // Build client.
    let mut client = Client::builder(
        configs::CONFIG.token.clone(),
        GatewayIntents::non_privileged(),
    )
    .event_handler(Handler)
    .await
    .expect("Error creating client!");

    // Finally, start up, print error if something went horrendously awry
    if let Err(why) = client.start().await
    {
        eprintln!("Error: {:?}", why);
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler
{
    async fn interaction_create(&self, ctx: Context, interaction: Interaction)
    {
        if let Interaction::ApplicationCommand(command) = interaction
        {
            command::run(ctx, command).await;
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready)
    {
        println!("{} is connected!", ready.user.name);

        for cmd in Command::get_global_application_commands(ctx.clone())
            .await
            .unwrap()
        {
            let _ = Command::delete_global_application_command(ctx.clone(), cmd.id).await;
        }

        let server = GuildId(configs::CONFIG.server);
        for cmd in GuildId::get_application_commands(&server, ctx.clone())
            .await
            .unwrap()
        {
            let _ = GuildId::delete_application_command(&server, ctx.clone(), cmd.id);
        }

        // I don't know a better way to do this
        GuildId::set_application_commands(&server, ctx, |commands| {
            commands
                .create_application_command(|command| {
                    command
                        .name("ping")
                        .description("A ping command, It responds if commands work.")
                })
                .create_application_command(|command| {
                    command
                        .name("id")
                        .description("Get a user id")
                        .create_option(|option| {
                            option
                                .name("id")
                                .description("The user to lookup")
                                .kind(CommandOptionType::User)
                                .required(true)
                        })
                })
                .create_application_command(|command| conversions::temp::register(command))
                .create_application_command(|command| time::timeh::register(command))
                .create_application_command(|command| meta::info::register(command))
                .create_application_command(|command| meta::license::register(command))
                .create_application_command(|command| {
                    randomize::random_choice::coin::register(command)
                })
                .create_application_command(|command| {
                    randomize::random_choice::roulette::register(command)
                })
                .create_application_command(|command| wiki::wiki::register(command))
                // .create_application_command(|command| currency::register(command))
        })
        .await
        .unwrap();
    }
}
