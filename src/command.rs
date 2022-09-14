use serenity::{
    model::{
        application::interaction::application_command::CommandDataOptionValue,
        prelude::interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
    },
    prelude::Context,
};

use super::extentions::{
    conversions::{
        temp,
    }
};

#[macro_export]
macro_rules! app_commands {
    ($ctx:expr) => {
        GuildId::set_application_commands(
            &GuildId(configs::CONFIG.server),
            &$ctx.http,
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
                    .create_application_command(|command| {
                        command
                            .name("temp")
                            .description("Convert from one temperature unit to another")
                            .create_option(|option| {
                                option
                                    .name("value")
                                    .description("Original value (e.g. '65F' [Fahrenheit], '18.33C' [Celsius].")
                                    .kind(CommandOptionType::String)
                                    .required(true)
                            })
                            .create_option(|option| {
                                option
                                    .name("target")
                                    .description("The unit to target. (e.g 'F' [Fahrenheit], 'C' [Celsius]).")
                                    .kind(CommandOptionType::String)
                                    .required(true)
                            })
                    })
            },
        )
        .await
    };
}

pub async fn run(ctx: Context, command: ApplicationCommandInteraction)
{
    let content = match command.data.name.as_str()
    {
        "ping" => "LMR Bot is alive".to_string(),
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
            let mut value:String = String::new();
            let mut target:char = '\0';

            if command.data.options.len() < 2
            {
                panic!("Expected User Arguments '[Value] [Target]'")
            }


            if let CommandDataOptionValue::String(_value) = command.data.options[0].resolved.as_ref().expect("Expected User Object")
            {
                value = _value.clone();
            }

            if let CommandDataOptionValue::String(_value) = command.data.options[1].resolved.as_ref().expect("Expected User Object")
            {
                target = _value.chars().last().unwrap();
            }
            temp::run(value, target)

        }

        _ => "not implemented :(".to_string(),
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
