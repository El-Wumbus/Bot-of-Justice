use once_cell::sync::Lazy;
use serde::Serialize;
use std::{env, fs::{read_to_string, write}, path::PathBuf, io::{stdin, stdout, Write}};
use serde_derive::Deserialize;
pub static CONFIG: once_cell::sync::Lazy<Config> = Lazy::new(||Config::parse());

#[derive(Deserialize, Serialize)]
pub struct Config
{
    pub token: String,
    pub server: u64,
    pub keys: Option<Keys>,
    pub behavior: Option<Behavior>,
}

#[derive(Deserialize, Serialize)]
pub struct Keys
{
    pub exchange_rate_api_key: Option<String>
}

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Behavior
{
    pub max_wiki_output: Option<usize>,
}

impl Config
{
    pub fn parse() -> Config
    {
        // Allow for changing config file location
        let config_file: PathBuf = match env::var("DISCORD_CONFIG")
        {
            Ok(x) => PathBuf::from(x),
            Err(_) => PathBuf::from("/etc/boj.conf"),
        };

        if !config_file.exists()
        {
            let id: u64;
            println!("First Time Setup");
            loop
            {
                print!("Enter your server id: ");
                stdout().flush().unwrap();
                let mut input:String = String::new();
                stdin().read_line(&mut input).unwrap();
                match input.trim().parse() {
                    Ok(x) => {
                        id = x;
                        
                        break;
                    },
                    Err(_) => {
                        println!("Not a proper id, try again!");
                        continue;
                    },
                };
            }
         
            print!("Enter your bot's token: ");
            stdout().flush().unwrap();
            let mut input:String = String::new();
            stdin().read_line(&mut input).unwrap();
            let token: String = input;

            
            let config = Config {
                token,
                server: id,
                keys: Some(Keys{exchange_rate_api_key:None}),
                behavior: Some(Behavior { max_wiki_output: None }),
            };

            let toml = toml::to_string(&config).unwrap();
            write(config_file.clone(), toml).unwrap();
        }

        let mut parsed:Config = toml::from_str(&read_to_string(config_file).unwrap()).unwrap();

        match env::var("DISCORD_TOKEN")
        {
            Ok(x) => parsed.token = x,
            Err(_) => (),
        };

        match env::var("DISCORD_SERVER")
        {
            Ok(x) => parsed.server = x.parse().expect("Provided Server isn't a positive integer value!"),
            Err(_) => (),
        };
        
        parsed
    }
}
