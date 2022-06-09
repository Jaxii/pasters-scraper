use clap::{App, Arg, SubCommand};
use linkify::{Link, LinkFinder, LinkKind};
use reqwest::{Client, StatusCode};
use std::borrow::Borrow;
use std::fmt::Debug;
use std::fs::File;
use std::io::{Error, Write};
use std::path::Path;
use std::{env, sync::Arc, time::Duration};
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;
use url::Url;

#[tokio::main]
async fn main() {

    File::create("urls.txt").unwrap();
    let client = Client::new();

    gen_url(client).await;
}

async fn get_html() -> Result<(String), Box<dyn std::error::Error>> {
    let html = reqwest::get("https://paste.rs/Aaa").await?.text().await?;
    Ok((html))
}

async fn parse_and_write(data: String) {
    let finder = LinkFinder::new();
    let links: Vec<Link> = finder.links(&data).collect();
    let mut file = File::options().append(true).open("urls.txt").unwrap();
    for i in links {
        let mut dat = i.as_str().as_bytes().to_vec();
        dat.push(0x0au8);
        file.write_all(&*dat).expect("TODO: panic message");
    }
}

async fn gen_url(client: Client) {
    const STRING: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
    let mut num = 0;
    for aa in STRING.chars() {
        for bb in STRING.chars() {
            for cc in STRING.chars() {
                let composed_str = format!("https://paste.rs/{}{}{}", aa, bb, cc);
                let response = client.get(composed_str).send().await.unwrap();
                if response.status() == StatusCode::OK {
                    let a = response.text().await.ok().expect("wtf?");

                    parse_and_write(a).await;

                    num += 1;
                    println!("Count: {}/238,328", num);
                } else if response.status().as_str() != "404"{
                    println!("sleeping");
                    sleep(Duration::from_secs(12)).await;

                }

                // println!("{}", parsed.unwrap());
            }
        }
    }
}
