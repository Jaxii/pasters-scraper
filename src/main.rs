use std::{env, sync::Arc, time::Duration};
use std::borrow::Borrow;
use std::fmt::Debug;
use std::fs::File;
use std::io::{Error, Write};
use std::path::Path;

use clap::{App, Arg, SubCommand};
use linkify::{Link, LinkFinder, LinkKind};
use reqwest::{Client, ClientBuilder, StatusCode};
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;
use url::Url;

#[tokio::main]
async fn main() {

    const STRING: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::http("http://203.30.189.196:80").unwrap())
        .proxy(reqwest::Proxy::http("http://203.28.9.142:80").unwrap())
        .proxy(reqwest::Proxy::http("http://45.12.31.171:80").unwrap()).build().expect("error?");

    File::create("urls.txt").unwrap();
    let mut num = 0;
    for aa in STRING.chars() {
        for bb in STRING.chars() {
            for cc in STRING.chars() {
                do_thing(format!("https://paste.rs/{}{}{}", cc, bb, aa),&client).await;
                num+=1;
                println!("{} / 238328", num);
            }
        }
    }


}

async fn do_thing(input: String, requester: &Client) {
    let response = requester.get(input).send().await.unwrap();
    if response.status() == StatusCode::OK {
        let a: Option<String> = response.text().await.ok();
        if a != None {
            parse_and_write(a.unwrap()).await;
        }
        sleep(Duration::from_millis(150)).await; //sleepy because rust is too blazingly🚀🔥🚀🔥🚀🚀🔥 fast
    } else if response.status().as_str() != "404" {
        println!("sleeping");
        sleep(Duration::from_secs(30)).await;
    }
}

async fn parse_and_write(data: String) {
    let finder = LinkFinder::new();
    let links: Vec<Link> = finder.links(&data).collect();
    let mut file = File::options().append(true).open("urls.txt").unwrap();
    for i in links {
        let mut dat = i.as_str().as_bytes().to_vec();
        dat.push(0x0au8); // new line character in bytes
        file.write_all(&*dat).expect("TODO: panic message");
    }
}