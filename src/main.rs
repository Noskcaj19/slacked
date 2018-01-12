#[macro_use]
extern crate failure;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;
extern crate ws;

use std::fs::File;
use std::io::Read;
use std::env;

use failure::Error;
// use failure::Fail;

mod errors;
mod objects;

use objects::{Config, Connect};
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

fn load_config() -> Result<Config, Error> {
    let home_dir = env::home_dir().ok_or(HomeDirError)?;
    let mut file = File::open(home_dir.join(".slacked.toml"))?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let config: Config = toml::from_str(&buf)?;
    Ok(config)
}

fn get_ws_url(token: &str) -> Result<String, Error> {
    let mut resp = reqwest::get(&format!(
        "https://slack.com/api/rtm.connect?token={}",
        token
    ))?;
    let result: Connect = resp.json()?;
    if !result.ok {
        bail!(RTMConnectError)
    }
    Ok(result.url)
}

fn connect_ws(ws_url: String) -> Result<(), Error> {
    ws::connect(ws_url, |out| Client { out: out })?;
    Ok(())
}

struct Client {
    out: ws::Sender,
}

impl ws::Handler for Client {
    fn on_message(&mut self, msg: ws::Message) -> Result<(), ws::Error> {
        println!("< {}", msg);
        Ok(())
    }
}

fn main() {
    let config = unwrap_or_exit!(load_config(), "Error loading config!");
    let ws_url = unwrap_or_exit!(get_ws_url(&config.token), "Error connecting to Slack");
    unwrap_or_exit!(connect_ws(ws_url), "Error connecting to Slack");
}
