use crate::configs::*;
use serenity::{builder::CreateApplicationCommand, model::prelude::command::CommandOptionType};
use truncrate::*;
use wikipedia;

pub fn run(search_term: String, id: bool) -> String
{
    return wiki_summary(get_wiki_id(search_term, id));
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand
{
    command
        .name("wiki")
        .description("Get a summary of a topic from wikipedia.org.")
        .create_option(|option| {
            option
                .name("search_term")
                .description("The term (or wikipedia page id) to search wikipedia.org for.")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("use_id")
                .description("Is the search term a wikipedia.org id? [default: false]")
                .kind(CommandOptionType::Boolean)
                .required(false)
        })
}

fn wiki_summary(id: String) -> String
{
    let handle = wikipedia::Wikipedia::<wikipedia::http::hyper::Client>::default();
    let page = handle.page_from_pageid(id);
    let mut max = match &CONFIG.behavior
    {
        None => 600,
        Some(x) => x.max_wiki_output.unwrap_or(600),
    };

    if max > 1000
    {
        max = 1000
    }

    let mut content = match page.get_summary()
    {
        Ok(x) => x,
        Err(x) => format!("Error: {}", x),
    };

    let title = match page.get_title()
    {
        Ok(x) => x,
        Err(x) => return format!("Error: {}", x),
    };

    if content.len() >= max
    {
        content = format!("{}...", content.truncate_to_boundary(max));
    }

    format!(
        "{}\nhttps://en.wikipedia.org/wiki/{}",
        content,
        title.trim().replace(" ", "_")
    )
}

fn get_wiki_id(search_term: String, id: bool) -> String
{
    return match id
    {
        true => search_term,
        false =>
        {
            let pageid = match wikipedia::Wikipedia::<wikipedia::http::hyper::Client>::default()
                .page_from_title(search_term)
                .get_pageid()
            {
                Ok(x) => x,
                Err(x) => format!("Error: Couldn't find wikipedia page `{}`", x),
            };
            pageid
        }
    };
}

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn get_wiki_id_search_test()
    {
        assert_eq!(get_wiki_id("Linux".to_string(), false), "6097297");
    }

    #[test]
    fn get_wiki_id_id_test()
    {
        assert_eq!(get_wiki_id("6097297".to_string(), true), "6097297");
    }
}
