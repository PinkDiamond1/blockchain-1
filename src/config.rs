extern crate toml;

use std::env;
use std::result::Result;
use std::io::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Deserialize)]
pub struct Config {
    api: Api,
    db: Db
}

#[derive(Deserialize)]
pub struct Api {
    address: Option<String>,
    keys_path: Option<String>,
    peer_address: Option<String>,
    peers: Option<Vec<String>>
}

#[derive(Deserialize)]
pub struct Db {
    path: Option<String>
}

impl Config {
    pub fn api(self) -> Api {
        self.api
    }
    pub fn db(self) -> Db {
        self.db
    }
}

impl Api {
    pub fn address(self) -> String {
        match env::var("API_ADDRESS") {
            Ok(value) => value,
            Err(_) => self.address.unwrap()
        }
    }
    pub fn keys_path(self) -> String {
        match env::var("API_KEYS_PATH") {
            Ok(value) => value,
            Err(_) => self.keys_path.unwrap()

        }
    }
    pub fn peer_address(self) -> String {
        match env::var("API_PEER_ADDRESS") {
            Ok(value) => value,
            Err(_) => self.peer_address.unwrap()

        }
    }
    pub fn peers(self) -> Vec<String> {

        match env::var("API_PEERS") {
            Ok(value) => {vec![]}, // todo: add parse environment
            Err(_) => self.peers.unwrap()
        }
    }
}

impl Db {
    pub fn path(self) -> String {
        match env::var("DB_PATH") {
            Ok(value) => value,
            Err(_) => self.path.unwrap()
        }
    }
}


///
/// Load configuration
///
/// # Examples
///
/// ```
/// assert_eq!("0.0.0.0:8000", read_config().ok().unwrap().api.address.unwrap().as_str())
/// ```
pub fn read_config() -> Result<Config, Error> {
    let mut content: String = String::new();
    let mut f = File::open(Path::new("./etc/config.toml"))?;
    f.read_to_string(&mut content);
    Ok(toml::from_str(content.as_str()).unwrap())
}

pub fn config() -> Config {
    read_config().ok().unwrap()
}

#[test]
fn positive() {
    assert_eq!("0.0.0.0:8000", config().api.address.unwrap().as_str())
}

#[test]
fn env_positive() {
    let address = "1.1.1.1:1231";
    env::set_var("API_ADDRESS", address);
    assert_eq!(address, config().api.address().as_str())
}