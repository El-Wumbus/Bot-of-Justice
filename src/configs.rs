use once_cell::sync::Lazy;
use serde::Serialize;
use serde_derive::Deserialize;
use std::{
    env,
    fs::{read_to_string, write, self},
    io::{stdin, stdout, Write},
    path::PathBuf,
};

pub static CONFIG: once_cell::sync::Lazy<Config> = Lazy::new(|| Config::parse());
pub static BLOCKLIST: once_cell::sync::Lazy<crate::extentions::simple::echo::Wordlist> =
    Lazy::new(|| parse_restricted_words());
const BLOCKED_WORDS_FILE: &str = "/etc/boj/blocked-words.json";
const MAIN_CONFIG_FILE: &str = "/etc/boj/config.toml";

structstruck::strike! {
    #[strikethrough[derive(Deserialize, Serialize)]]
    #[strikethrough[serde(rename_all = "camelCase")]]
    pub struct Config
    {
        /// The discord application token.
        pub token: String,

        /// The discord server's ID
        pub server: u64,

        /// API keys used in the program for some functions
        pub keys: struct
        {
            pub exchange_rate_api_key: String,
        },

        /// Optional configurable behavior for the bot.
        pub behavior: Option<Behavior>,
    }
}

#[derive(Deserialize, Serialize)]
pub struct Behavior
{
    /// The maximum number of characters allowed for wiki output
    pub max_wiki_output: Option<usize>,

    /// Defines if the log output should be saved to a file
    pub log_to_file: Option<bool>,
}

impl Config
{
    pub fn parse() -> Config
    {
        // Allow for changing config file location
        let config_file: PathBuf = match env::var("BOJ_CONFIG")
        {
            Ok(x) => PathBuf::from(x),
            Err(_) => PathBuf::from(MAIN_CONFIG_FILE),
        };

        if !config_file.exists()
        {
            let id: u64;
            println!("First Time Setup");
            loop
            {
                print!("Enter your server id: ");
                stdout().flush().unwrap();
                let mut input: String = String::new();
                stdin().read_line(&mut input).unwrap();
                match input.trim().parse()
                {
                    Ok(x) =>
                    {
                        id = x;

                        break;
                    }
                    Err(_) =>
                    {
                        println!("Not a proper id, try again!");
                        continue;
                    }
                };
            }

            print!("Enter your bot's token: ");
            stdout().flush().unwrap();
            let mut input: String = String::new();
            stdin().read_line(&mut input).unwrap();
            let token: String = input;

            print!("Enter your exchange rate api key: ");
            stdout().flush().unwrap();
            let mut input: String = String::new();
            stdin().read_line(&mut input).unwrap();
            let key: String = input;

            let config = Config {
                token,
                server: id,
                keys: Keys{exchange_rate_api_key: key},
                behavior: Some(Behavior {
                    max_wiki_output: None,
                }),
            };

            let toml = toml::to_string(&config).unwrap();
            fs::create_dir_all(config_file.parent().unwrap()).unwrap();
            write(config_file.clone(), toml).unwrap();
        }

        let mut parsed: Config = toml::from_str(&read_to_string(config_file).unwrap()).unwrap();

        match env::var("BOJ_TOKEN")
        {
            Ok(x) => parsed.token = x,
            Err(_) => (),
        };

        match env::var("BOJ_SERVER")
        {
            Ok(x) =>
            {
                parsed.server = x
                    .parse()
                    .expect("Provided Server isn't a positive integer value!")
            }
            Err(_) => (),
        };

        parsed
    }

}

fn parse_restricted_words() -> crate::extentions::simple::echo::Wordlist
    {
        let config_file: PathBuf = match env::var("BOJ_BLOCKED_WORDS")
        {
            Ok(x) => PathBuf::from(x),
            Err(_) => PathBuf::from(BLOCKED_WORDS_FILE),
        };

        // First time setup
        if !config_file.exists()
        {
            // Ask for a wordlist
            println!("Enter a comma separated list of blocked words: ");
            stdout().flush().unwrap();
            let mut input: String = String::new();
            stdin().read_line(&mut input).unwrap();

            // Make a wordlist from the input
            let mut words: Vec<String> = Vec::new();
            for word in input.split(",")
            {
                words.push(String::from(word.to_lowercase()));
            }

            let blocklist = crate::extentions::simple::echo::Wordlist {
                words,
            };

            // Serialize the blocklist as toml
            let toml = serenity::json::prelude::to_string(&blocklist).unwrap();

            fs::create_dir_all(config_file.parent().unwrap()).unwrap();
            // Write the toml to the config file
            fs::write(config_file.clone(), toml).unwrap();
            return blocklist;
        }

        // Parse from the file
        serenity::json::prelude::from_str(&read_to_string(config_file).unwrap()).unwrap()
    }