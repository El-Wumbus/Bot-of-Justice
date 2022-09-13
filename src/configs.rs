use once_cell::sync::Lazy;
use std::{env, collections::HashMap};

pub static CONFIG: once_cell::sync::Lazy<Config> = Lazy::new(|| Config::parse());

pub struct Config
{
    pub token: String,
    pub server: u64,
}

impl Config
{
    fn parse() -> Config
    {
        let token: String;
        let server: u64;

        // Allow for changing config file location
        let config_file: String = match env::var("PMR_CONFIG")
        {
            Ok(x) => x,
            Err(_) => String::from("/etc/lmr.conf"),
        };
        let parse_config = |config_file: &str| -> HashMap<String, HashMap<String, Option<String>>> {
            ini!(config_file)
        };

        token = match env::var("PMR_TOKEN")
        {
            Ok(x) => x,
            Err(_) => match parse_config(&config_file)["bot"]["token"].clone()
            {
                Some(x) => x,
                None => String::from(""),
            },
        };

        server = match env::var("PMR_SERVER")
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

        Config { token, server }
    }
}
