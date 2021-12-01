use std::collections::HashMap;
use std::fs::{remove_file, File, OpenOptions};
use std::io::prelude::*;

use anyhow::{anyhow, Result};
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[command]
pub async fn tag(ctx: &Context, msg: &Message) -> CommandResult {
    let content: Vec<&str> = msg.content.trim().split(" ").collect();
    let file = File::open("tags.ron");
    let mut tags: HashMap<&str, &str> = HashMap::new();
    let mut serialized = String::new();
    if file.is_ok() {
        file?.read_to_string(&mut serialized)?;
        tags = ron::from_str(&serialized)?;
    }
    if content.len() == 1 {
        msg.channel_id
            .say(ctx, "!tag { add, remove, \\_ }\n\\_ = tag name")
            .await?;
        return Ok(());
    }
    let result = match content[1] {
        "add" => add(content, tags),
        "remove" => remove(content, tags),
        _ => {
            if tags.get(content[1]).is_none() {
                msg.reply(ctx, "That tag does not exist.");
                return Ok(());
            } else {
                msg.channel_id
                    .say(ctx, tags.get(content[1]).unwrap())
                    .await?;
                return Ok(());
            }
        }
    };
    match result {
        Ok(tags) => {
            remove_file("tags.ron");
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open("tags.ron")?;
            file.write(ron::to_string(&tags).unwrap().as_bytes())
                .unwrap();
            msg.reply(ctx, "Done!").await;
            return Ok(());
        }
        Err(err) => {
            msg.reply(ctx, err).await;
            return Ok(());
        }
    }
}

// Add a provided entry to tags from content's index 2 as the key and index 3 and beyond joined as a space seperated string.
// If the content doesn't provide a key or content, or the key is 'add' or 'remove' then err, otherwise ok with updated tags.
fn add<'a>(
    content: Vec<&'a str>,
    mut tags: HashMap<&'a str, &'a str>,
) -> Result<HashMap<&'a str, &'a str>> {
    if content.len() <= 2 {
        return Err(anyhow!(
            "Tag cannot be null\n !tag add {{name}} {{content}}"
        ));
    }
    if content[2] == "add" || content[2] == "remove" {
        return Err(anyhow!(
            "Tag cannot be 'add' or 'remove'\n !tag add {{name}} {{content}}"
        ));
    }
    tags.insert(&content[2], &content[3..].join(" ")[..]);
    Ok(tags)
}

/// Remove a provided entry from tags by it's key given from the second index of content.
/// If the content doesn't provide a name then Err. Otherwise Ok with updated tags.
fn remove<'a>(
    content: Vec<&str>,
    mut tags: HashMap<&'a str, &'a str>,
) -> Result<HashMap<&'a str, &'a str>> {
    if content.len() <= 2 {
        return Err(anyhow!(
            "Tag cannot be null\n !tag add {{name}} {{content}}"
        ));
    }
    if tags.remove(content[2]).is_none() {
        return Err(anyhow!("That tag doesn't exist."));
    }
    Ok(tags)
}
