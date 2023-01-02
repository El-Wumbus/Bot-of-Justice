#![allow(non_snake_case)]
mod command;
mod configs;
mod extentions;

use chrono::Utc;
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
use tokio::spawn;
use tokio_schedule::{every, Job};
use wd_log::{self, log_info_ln, log_error_ln};


const AUTHOR: &str = "Decator";
const GITHUB: &str = "https://github.com/El-Wumbus/Bot-of-Justice";
const VERSION: &str = "0.4.0";

async fn fetch_api_keys() {
    match extentions::conversions::currency::ExchangeRates::fetch().await
            {
                Ok(_) => log_info_ln!("Succesfully fetched echange rates api key"),
                Err(x) => log_error_ln!("Couldn't fetch echange rates api key: {x}"),
            };
}

#[tokio::main]
async fn main()
{
    wd_log::set_level(wd_log::INFO);
    wd_log::set_prefix("BOJ_LOG");
    wd_log::show_time(true);
    wd_log::show_file_line(false);

    log_info_ln!(
        "Starting BOJ (Bot of Justice) Version {}. Written by {}. See the source code at '{}'",
        VERSION, AUTHOR, GITHUB
    );

    // Schedule echange rate fetching
    fetch_api_keys().await;
    let fetch_exchange_rates = every(6)
        .hour()
        .at(0, 0)
        .in_timezone(&Utc)
        .perform(|| async {
            fetch_api_keys().await;
        });
    spawn(fetch_exchange_rates);

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
        log_error_ln!("Error: {:?}", why);
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
        log_info_ln!("{} is connected!", ready.user.name);

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
                .create_application_command(|command| simple::echo::register(command))
                .create_application_command(|command| conversions::currency::register(command))
        })
        .await
        .unwrap();
    }
}
