#[macro_use]
extern crate failure;
extern crate linenoise;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate slack_api;
extern crate toml;
extern crate ws;

use std::fs::File;
use std::io::Read;
use std::env;
use std::thread;
use std::sync::mpsc;

use failure::Error;

mod errors;
mod api_items;
mod ws_client;

use errors::*;

macro_rules! unwrap_or_exit {
    ($c:expr, $($e:expr),+) => {
        match $c {
            Ok(x) => x,
            Err(e) => {
                eprintln!($($e),+);
                eprintln!("{}", e.cause());
                std::process::exit(1);
            }
        };
    }
}

fn load_config() -> Result<api_items::Config, Error> {
    let home_dir = env::home_dir().ok_or(HomeDirError)?;
    let mut file = File::open(home_dir.join(".slacked.toml"))?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let config: api_items::Config = toml::from_str(&buf)?;
    Ok(config)
}

fn get_ws_url(token: &str) -> Result<String, Error> {
    let mut resp = reqwest::get(&format!(
        "https://slack.com/api/rtm.connect?token={}",
        token
    ))?;
    let result: api_items::Connect = resp.json()?;
    if !result.ok {
        bail!(RTMConnectError)
    }
    Ok(result.url)
}

fn connect_ws(ws_url: String, tx: mpsc::Sender<ws_client::WSEvent>) -> Result<(), Error> {
    ws::connect(ws_url, |out| ws_client::Client {
        out: out,
        status: tx.clone(),
    })?;
    Ok(())
}

fn main() {
    let config = unwrap_or_exit!(load_config(), "Error loading config!");
    let ws_url = unwrap_or_exit!(get_ws_url(&config.token), "Error connecting to Slack");

    let (tx, rx) = mpsc::channel();

    let client_thread = thread::spawn(move || {
        unwrap_or_exit!(connect_ws(ws_url, tx.clone()), "Error connecting to Slack");
    });

    if let Ok(ws_client::WSEvent::Connected(client)) = rx.recv() {
        loop {
            match linenoise::input("> ") {
                Some(line) => {
                    linenoise::history_add(&line);
                    println!("Read line: {}", line)
                }
                None => {
                    client.close().unwrap();
                    break;
                }
            };
        }
    };
    client_thread.join().unwrap();
}
