use serenity::{client::Context, framework::{
        standard::{
            CommandResult, 
            macros::command
            }
        }, model::{channel::Message}};

#[command]
pub async fn echo(ctx: &Context, msg: &Message) -> CommandResult{
    msg.channel_id.say(
        ctx, 
        &msg.content_safe(ctx).await[6..]
    ).await?;
    Ok(())
}