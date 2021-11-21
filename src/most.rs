use serenity::{
    Client, 
    client::Context, 
    framework::{
        StandardFramework, 
        standard::{
            CommandResult, 
            macros::{
                group, 
                command}
            }
        }, 
        model::channel::Message};

#[command]
pub async fn echo(ctx: &Context, msg: &Message) -> CommandResult{
    msg.reply(ctx, &msg.content[4..]).await?;
    Ok(())
}