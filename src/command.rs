use serenity::{
    model::{
        application::interaction::application_command::CommandDataOptionValue,
        prelude::interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
    },
    prelude::Context,
};

use super::extentions::{conversions::temp, wiki::wiki};
use crate::extentions::meta::{license::*, self};
use crate::{
    extentions::randomize::random_choice::{coin, roulette},
};

pub async fn run(ctx: Context, command: ApplicationCommandInteraction)
{
    let content = match command.data.name.as_str()
    {
        "ping" => "Bot is alive".to_string(),
        "id" =>
        {
            let options = command
                .data
                .options
                .get(0)
                .expect("Expected user option")
                .resolved
                .as_ref()
                .expect("Expected user object");

            if let CommandDataOptionValue::User(user, _member) = options
            {
                format!("{}'s id is {}", user.tag(), user.id)
            }
            else
            {
                "Please provide a valid user".to_string()
            }
        }

        "temp" =>
        {
            let mut value: String = String::new();
            let mut target: char = '\0';

            if command.data.options.len() < 2
            {
                panic!("Expected User Arguments '[Value] [Target]'")
            }

            if let CommandDataOptionValue::String(_value) = command.data.options[0]
                .resolved
                .as_ref()
                .expect("Expected User Object")
            {
                value = _value.clone();
            }

            if let CommandDataOptionValue::String(_value) = command.data.options[1]
                .resolved
                .as_ref()
                .expect("Expected User Object")
            {
                target = _value.chars().last().unwrap();
            }
            temp::run(value, target)
        }
        "wiki" =>
        {
            let mut search_term: String = String::new();
            let mut use_id: bool = false;

            if let CommandDataOptionValue::String(_value) = command.data.options[0]
                .resolved
                .as_ref()
                .expect("Expected User Object")
            {
                search_term = _value.clone();
            }

            if command.data.options.len() > 1
            {
                if let CommandDataOptionValue::Boolean(_value) = command.data.options[1]
                    .resolved
                    .as_ref()
                    .expect("Expected User Object")
                {
                    use_id = *_value;
                }
            }

            wiki::run(search_term, use_id)
        }

        "coin" => coin::run(),

        "roulette" => roulette::run(),

        "linfo" => if let CommandDataOptionValue::String(_value) =
            command.data.options[1]
                .resolved
                .as_ref()
                .expect("Expected User Object")
        {
            match _value.to_lowercase().as_str()
            {
                "gplv3" => License::GPLV2,
                "mit" => License::MIT,
                _ => "That license is not in our record 😭",
            }
        }
        else
        {
            "Error!"
        }
        .to_string(),

        "info" => meta::info::run(),
        
        _ => "not a thing, bozo 🤓.\nL + nerd".to_string(),
    };

    if let Err(why) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content(content))
        })
        .await
    {
        println!("Cannot respond to slash command: {}", why);
    }
}
