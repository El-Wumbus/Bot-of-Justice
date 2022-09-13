use serenity::{
    model::{
        application::interaction::application_command::CommandDataOptionValue,
        prelude::interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
    },
    prelude::Context,
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
        "attachmentinput" =>
        {
            let options = command
                .data
                .options
                .get(0)
                .expect("Expected attachment option")
                .resolved
                .as_ref()
                .expect("Expected attachment object");

            if let CommandDataOptionValue::Attachment(attachment) = options
            {
                format!(
                    "Attachment name: {}, attachment size: {}",
                    attachment.filename, attachment.size
                )
            }
            else
            {
                "Please provide a valid attachment".to_string()
            }
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
