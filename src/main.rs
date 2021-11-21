use std::io::prelude::*;
use std::fs::File;

use anyhow::Result;
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

mod most;
use most::*;

#[group]
#[commands("echo")]
struct Most;

#[tokio::main]
async fn main() -> Result<()> {
    let mut file = File::open("/etc/hosts")?;
    let mut token = String::new();
    file.read_to_string(&mut token)?;

    let framework = StandardFramework::new()
    .configure(|c|{
        c.prefix("!")
    })
    .group(&MOST_GROUP);
    let mut client = Client::builder(token).framework(framework).await?;
    client.start().await?;

    return Ok(());

}