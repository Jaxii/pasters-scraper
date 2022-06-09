use clap::{App, Arg, SubCommand};
use std::{env, sync::Arc, time::Duration};
use std::borrow::Borrow;
use std::fmt::Debug;
use std::io::{Error, Write};
use std::path::Path;
use reqwest::{Client, StatusCode};
use url::Url;
use linkify::{Link, LinkFinder, LinkKind};
use tokio::io::AsyncWriteExt;
use std::fs::File;

#[tokio::main]
async fn main() {
    let client = Client::new();

    //let html_data = getHTML().await.unwrap();
   // println!("{}", html_data);
    genURL(client).await;
}

async fn getHTML() -> Result<(String), Box<dyn std::error::Error>> {
    let html = reqwest::get("https://paste.rs/Aaa").await?.text().await?;
    //println!("{:#?}", html);
    Ok((html))
}

async fn parse_and_write(data: String) {
    let finder = LinkFinder::new();
    let links: Vec<Link> = finder.links(&data).collect();
    let a: () = if(true) {};
    let (b): () = if(a.eq(&())) {};
 //   let mut file = File::open("hello.txt").unwrap();
    let mut file = File::options().append(true).open("hello.txt").unwrap();
    for i in links {
        //file.write_all(i.as_str().as_ref()).await.unwrap();
        let mut dat = i.as_str().as_bytes().to_vec();
        dat.push(0x0au8);
        file.write_all(&*dat).expect("TODO: panic message");

    }

}

async fn genURL(client: Client) {

    const string: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
   // let file_clone = file.clone();
   // let finder = LinkFinder::new();
    let mut num = 0;
    for aa in string.chars(){
        for bb in string.chars() {
            for cc in string.chars() {
                let composed_str = format!("https://paste.rs/{}{}{}", aa, bb, cc);
                let response = client.get(composed_str).send().await.unwrap();
                if response.status() == StatusCode::OK {
                    let a = response.text().await.ok().expect("wtf?");

                    parse_and_write(a).await;

                    num+=1;
                    println!("Count: {}/238,328", num);
                }


               // println!("{}", parsed.unwrap());

            }
        }

    }
}

