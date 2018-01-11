#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate ws;

use std::fs::File;
use std::io::Read;
use std::env;

use failure::Error;

mod errors;
use errors::*;

#[derive(Deserialize, Debug)]
struct Config {
    token: String,
}

fn load_config() -> Result<Config, Error> {
    let home_dir = env::home_dir().ok_or(HomeDirError)?;
    let mut file = File::open(home_dir.join(".slacked.toml"))?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let config: Config = toml::from_str(&buf)?;
    Ok(config)
}

fn main() {
    let config = match load_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading config!");
            eprintln!("{}", e.cause());
            std::process::exit(1);
        }
    };
    println!("{:?}", config);
    println!("Hello, world!");
}
