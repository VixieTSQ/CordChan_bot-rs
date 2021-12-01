use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[command]
pub async fn echo(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(ctx, &msg.content_safe(ctx).await[6..])
        .await?;
    Ok(())
}
