
use serde::Deserialize;
use serde::Serialize;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;

#[derive(Deserialize, Clone, Serialize)]
pub struct Wordlist
{
    pub words: Vec<String>,
}

fn check_words(input: String) -> Result<String, ()>
{
    let blocklist = crate::configs::BLOCKLIST.clone();

    for word in blocklist.words
    {
        // Check the lowercase input against the lowercase blocked word.
        if input.to_lowercase().contains(&word.to_lowercase())
        {
            // Return failure if the restricted word is found.
            return Err(());
        }
    }

    // Pass the input back the the caller.
    Ok(input)
}

pub fn run(input: String) -> String
{
    match check_words(input)
    {
        Ok(x) => x,
        _ => "A restricted word was used!".to_string(),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand
{
    command
        .name("echo")
        .description("Repeat something")
        .create_option(|option| {
            option
                .name("input")
                .description("The message to repeat")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
