
pub mod coin
{
    use serenity::builder::CreateApplicationCommand;

    pub fn run() -> String
    {
        match rand::random::<bool>()
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
    use rand::Rng;
    use serenity::builder::CreateApplicationCommand;

    pub fn run() -> String
    {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=5)
        {
            3 => "Dead".to_string(),
            _=> "Alive".to_string(),
        }
    }

    pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand
    {
    
        command
            .name("roulette")
            .description("Pull the tigger, 1/6 chance you die")
    }
}
