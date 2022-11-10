use serenity::{builder::CreateApplicationCommand, model::prelude::command::CommandOptionType};

enum Temperature
{
    Kelvin(f64),
    Celsius(f64),
    Fahrenheit(f64),
}

impl From<Temperature> for f64
{
    fn from(item: Temperature) -> Self
    {
        match item
        {
            Temperature::Kelvin(x) => x,
            Temperature::Celsius(x) => x,
            Temperature::Fahrenheit(x) => x, 
        }
    }
}

impl Temperature
{
    pub fn to_fah(&self) -> Temperature
    {
        match *self
        {
            Self::Kelvin(x) => Self::Fahrenheit((1.8 * (x - 273.15)) + 32.0),
            Self::Celsius(x) => Self::Fahrenheit(x * 1.8 + 32 as f64),
            Self::Fahrenheit(x) => Self::Fahrenheit(x),
        }
    }
    pub fn to_cel(&self) -> Temperature
    {
        match *self
        {
            Self::Kelvin(x) => Self::Celsius(x - 273.15),
            Self::Fahrenheit(x) => Self::Celsius((x - 32.0) / 1.8),
            Self::Celsius(x) => Self::Celsius(x),
        }
    }
    pub fn to_kel(&self) -> Temperature
    {
        match *self
        {
            Self::Fahrenheit(x) => Self::Kelvin(((x - 32.0) / 1.8) + 273.15),
            Self::Celsius(x) => Self::Kelvin(x + 273.15),
            Self::Kelvin(x) => Self::Kelvin(x),
        }
    }
}

pub fn run(value: String, target: char) -> String
{
    let mut value = value;
    let last = value.clone().chars().last().unwrap();
    let conval: Temperature;
    let ret: f64;
    value.pop();

    match last
    {
        'C' | 'c' =>
        {
            conval = Temperature::Celsius(value.parse().unwrap_or(0.0));
        }

        'F' | 'f' =>
        {
            conval = Temperature::Fahrenheit(value.parse().unwrap_or(0.0));
        }

        'K' | 'k' =>
        {
            conval = Temperature::Kelvin(value.parse().unwrap_or(0.0));
        }

        _ => return "Error: No viable unit specified".to_string(),
    }

    ret = match target
    {
        'F' | 'f' => f64::from(conval.to_fah()),
        'C' | 'c' => f64::from(conval.to_cel()),
        'K' | 'k' => f64::from(conval.to_kel()),
        _ => return "Error: No viable target specified".to_string(),
    };

    if f64::from(conval) == 0.0
    {
        return "Error: not a parseable number!".to_string();
    }

    format!("{:.1}{}", ret, target)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand
{
    command
        .name("temp")
        .description("Convert from one temperature unit to another. Supports Kelvin, Fahrenheit, and Celcius")
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
                .description("The unit to target. (e.g 'F' [Fahrenheit], 'K' [kelvin]).")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
