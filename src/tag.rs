use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::collections::HashMap;

use anyhow::Result;
use serenity::{client::Context, framework::{
    standard::{
        CommandResult, 
        macros::command
        }
    }, model::{channel::Message}};

#[command]
pub async fn tag(ctx: &Context, msg: &Message) -> CommandResult{
    let content: Vec<&str> = msg.content.trim().split(" ").collect();
    let mut serialized = String::new();
    let file = File::open("tags.json");
    let mut tags: HashMap<&str, &str> = HashMap::new();
    if file.is_err() {File::create("tags.json")?;} else {
        file?.read_to_string(&mut serialized)?;
        tags = serde_json::from_str(&serialized)?;
    }
    if content.len() == 1 {
        msg.channel_id.say(
            ctx, 
            "!tag { add, remove, \\_ }\n\\_ = tag name"
        ).await?;
        return Ok(());
    }
    match content[1] {
        "add" => add(ctx, msg, content, tags).await?,
        "remove" => remove(ctx, msg, content, tags).await?,
        _ => print(ctx, msg, content, tags).await?
    }
    Ok(())
}
async fn add(ctx: &Context, msg: &Message, content: Vec<&str>, mut tags: HashMap<&str, &str>) -> Result<()> {
    if content.len() <= 2 {
        msg.channel_id.say(
            ctx, 
            "Tag cannot be null\n !tag add {name} {content}"
        ).await?;
        return Ok(());
    }
    if content[2] == "add" || content[2] == "remove" {
        msg.channel_id.say(
            ctx, 
            "Tag cannot be 'add' or 'remove'\n !tag add {name} {content}"
        ).await?;
        return Ok(());
    }
    let tag_content = &content[3..].join(" ")[..];
    tags.insert(&content[2], &tag_content);
    let mut file = OpenOptions::new().write(true).open("tags.json")?;
    file.write(serde_json::to_string(&tags).unwrap().as_bytes()).unwrap();
    msg.reply(ctx, "Done!").await?;
    Ok(())
}
async fn remove(ctx: &Context, msg: &Message, content: Vec<&str>, mut tags: HashMap<&str, &str>) -> Result<()> {
    if content.len() <= 2 {
        msg.channel_id.say(
            ctx, 
            "Tag cannot be null\n !tag add {name} {content}"
        ).await?;
        return Ok(());
    }
    if tags.remove(content[2]).is_none() {
        msg.reply(ctx, "That tag doesn't exist.").await?;
        return Ok(());
    }
    let mut file = OpenOptions::new().write(true).open("tags.json")?;
    file.write(serde_json::to_string(&tags).unwrap().as_bytes()).unwrap();
    msg.reply(ctx, "Done!").await?;
    Ok(())
}
async fn print(ctx: &Context, msg: &Message, content: Vec<&str>, tags: HashMap<&str, &str>) -> Result<()> {
    if tags.get(content[1]).is_none() {
        msg.reply(ctx, "That tag doesn't exist.").await?;
        return Ok(());
    }
    msg.channel_id.say(
        ctx, 
        tags.get(content[1]).unwrap()
    ).await?;
    Ok(())
}