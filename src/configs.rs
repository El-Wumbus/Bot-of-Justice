use once_cell::sync::Lazy;
use std::{collections::HashMap, env};

pub static CONFIG: once_cell::sync::Lazy<Config> = Lazy::new(|| Config::parse());

pub struct Config
{
    pub token: String,
    pub server: u64,
    pub exchange_api_key: String,
    pub options: ConfigurationOptions
}

pub struct ConfigurationOptions
{}

impl Config
{
    pub fn parse() -> Config
    {
        let token: String;
        let server: u64;

        // Allow for changing config file location
        let config_file: String = match env::var("PMR_CONFIG")
        {
            Ok(x) => x,
            Err(_) => String::from("/etc/boj.conf"),
        };
        let parse_config = |config_file: &str| -> HashMap<String, HashMap<String, Option<String>>> {
            ini!(config_file)
        };

        token = match env::var("DISCORD_TOKEN")
        {
            Ok(x) => x,
            Err(_) => match parse_config(&config_file)["bot"]["token"].clone()
            {
                Some(x) => x,
                None => String::from(""),
            },
        };

        server = match env::var("DISCORD_SERVER")
        {
            Ok(x) => x,
            Err(_) => match parse_config(&config_file)["bot"]["server"].clone()
            {
                Some(x) => x,
                None => String::from("0"),
            },
        }
        .parse()
        .expect("Provided Server isn't a positive integer value!");
        
        let exchange_api_key = match env::var("DISCORD_EXCHANGE_API_KEY")
        {
            Ok(x) => x,
            Err(_) => match parse_config(&config_file)["keys"]["exchange"].clone()
            {
                Some(x) => x,
                None => String::from(""),
            },
        };

        
        Config 
        {
            token,
            server,
            exchange_api_key,
            options:ConfigurationOptions{ },
        }
    }
}
