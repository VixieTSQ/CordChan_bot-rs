#![allow(non_snake_case)]

use std::io::prelude::*;
use std::fs::File;

use anyhow::Result;
use serenity::{
    Client, 
    framework::{
        StandardFramework, 
        standard::{ 
            macros::group
            }
        }, };

mod most;
mod tag;
use most::*;
use tag::*;

#[group]
#[commands("echo")]
struct Most;

#[group]
#[commands("tag")]
struct Tag;

#[tokio::main]
async fn main() -> Result<()> {
    let mut file = File::open("secret")?;
    let mut token = String::new();
    file.read_to_string(&mut token)?;

    let framework = StandardFramework::new()
    .configure(|c|{
        c.prefix("!")
    })
    .group(&MOST_GROUP).group(&TAG_GROUP);
    let mut client = Client::builder(token).framework(framework).await?;
    client.start().await?;

    return Ok(());

}