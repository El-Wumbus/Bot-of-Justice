pub mod coin
{
    use serenity::builder::CreateApplicationCommand;

    pub fn run() -> String
    {
        let mut rng = rand::thread_rng();

        match rand::Rng::gen_bool(&mut rng, 1.0 / 2.0)
        {
            true => String::from("Heads"),
            false => String::from("Tails"),
        }
    }

    pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand
    {
        command
            .name("coin")
            .description("Flip a coin there's a 50% chance it's heads")
    }
}

pub mod roulette
{
    use serenity::builder::CreateApplicationCommand;

    pub fn run() -> String
    {
        let mut rng = rand::thread_rng();
        match rand::Rng::gen_bool(&mut rng, 1.0 / 6.0)
        {
            true => "Dead".to_string(),
            false => "Alive".to_string(),
        }
    }

    pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand
    {
        command
            .name("roulette")
            .description("Pull the tigger, 1/6 chance you die")
    }
}
