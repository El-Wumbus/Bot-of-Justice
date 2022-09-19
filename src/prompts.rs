
use serenity::{
  model::{prelude::{ChannelId, Message, ReactionType}, user::User},
  prelude::Context,
};
use serenity_utils::{prompt::reaction_prompt, prompt::yes_or_no_prompt, Error};


pub async fn reaction_prompt_bool(ctx: &Context, user: &User, prompt:String) -> Result<bool, Error> {
  let prompt_msg = ChannelId(7).say(&ctx.http, prompt).await?;
  // Result of user's reaction to the prompt.
  let result =  yes_or_no_prompt(ctx, &prompt_msg, user, 30.0).await?;
  match result
  {
    true => return Ok(true),
    false => return Ok(false),
  }
}


