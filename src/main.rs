#![allow(non_snake_case)]
mod configs;
mod command;
#[macro_use]
extern crate ini;
use serenity::{
    async_trait,
    model::{
        application::{
            command::{ CommandOptionType},
            interaction::Interaction,
        },
        gateway::Ready,
        id::GuildId,
    },
    prelude::*,
};

#[tokio::main]
async fn main()
{
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

        match app_commands!(ctx){
            Ok(_) => (),
            Err(x) => eprintln!("Error {}", x),
        };

    }
}
