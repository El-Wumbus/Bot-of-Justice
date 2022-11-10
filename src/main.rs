#![allow(non_snake_case)]
mod command;
mod extentions;
mod configs;
#[macro_use]
extern crate ini;
use serenity::{
    async_trait,
    model::{
        application::{command::CommandOptionType, interaction::Interaction},
        gateway::Ready,
        id::GuildId,
    },
    prelude::*,
};

const AUTHOR:&str = "Decator";
const GITHUB:&str = "https://github.com/El-Wumbus/Bot-of-Justice";
const VERSION:&str = "0.2.0";

#[tokio::main]
async fn main()
{
    println!("Starting BOJ (Bot of Justice) Version {}.\nWritten by {}. See the source code at '{}'",VERSION, AUTHOR, GITHUB) ;
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

        let commands = GuildId::set_application_commands(
            &GuildId(configs::CONFIG.server),
            &ctx,
            |commands| {
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
                .create_application_command(|command| extentions::conversions::temp::register(command))
                .create_application_command(|command| extentions::meta::info::register(command))
                .create_application_command(|command| extentions::meta::license::register(command))
                .create_application_command(|command| extentions::randomize::random_choice::coin::register(command))
                .create_application_command(|command| extentions::randomize::random_choice::roulette::register(command))
            },
        )
        .await;
    }
}
