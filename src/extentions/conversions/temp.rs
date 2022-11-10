use serenity::{builder::CreateApplicationCommand, model::prelude::command::CommandOptionType};

fn fah_to_cel(value: f64) -> f64 { (value - 32 as f64) / 1.8 }
fn cel_to_fah(value: f64) -> f64 { (value * 1.8) + 32 as f64 }

pub fn run(value: String, target: char) -> String
{
    let mut value = value;
    let last = value.clone().chars().last().unwrap();

    match last
    {
        'C' | 'c' =>
        {
            if target == 'C' || target == 'c'
            {
                return String::from(value);
            }
        }

        'F' | 'f' =>
        {
            if target == 'F' || target == 'f'
            {
                return String::from(value);
            }
        }

        _ => return "Error: No unit specified".to_string(),
    }

    value.pop();
    return parse_value(value, target);
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand
{
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
}

fn parse_value(value: String, conversion: char) -> String
{
    let value: f64 = match value.parse()
    {
        Ok(x) =>
        {
            let val: f64;
            match conversion
            {
                'C' | 'c' =>
                {
                    val = fah_to_cel(x);
                }
                'F' | 'f' =>
                {
                    val = cel_to_fah(x);
                }
                _ => val = 0 as f64,
            }
            val
        }
        Err(_) => 0 as f64,
    };

    if value == 0 as f64
    {
        return "Error: not a parseable number!".to_string();
    }

    return format!("{:.1}{}", value, conversion);
}
