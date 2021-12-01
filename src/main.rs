#![allow(non_snake_case)]

use std::fs::File;
use std::io::prelude::*;

use anyhow::Result;
use serenity::{
    framework::{standard::macros::group, StandardFramework},
    Client,
};

mod general;
mod tag;
use general::*;
use tag::*;

#[group]
#[commands("echo")]
struct General;

#[group]
#[commands("tag")]
struct Tag;

#[tokio::main]
async fn main() -> Result<()> {
    let mut file = File::open("secret")?;
    let mut token = String::new();
    file.read_to_string(&mut token)?;

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP)
        .group(&TAG_GROUP);
    let mut client = Client::builder(token).framework(framework).await?;
    client.start().await?;

    return Ok(());
}
