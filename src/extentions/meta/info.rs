use serenity::builder::CreateApplicationCommand;

use crate::{AUTHOR, GITHUB, VERSION};

pub fn run() -> String
{
    format!("Author: {}\n Github: {}\n\
    Bot of Justice Version `{}` , Copyright (C) 2022 Decator\n\
    Bot of Justice comes with ABSOLUTELY NO WARRANTY.\n\
    This is free software, and you are welcome to redistribute it\n\
    under certain conditions; for details use command `license GPLv2`", AUTHOR, GITHUB, VERSION)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand
{
    command
        .name("info")
        .description("Display bot info")
}