use std::convert::From;
use std::fs::File;
use std::io::Read;

use serde::Deserialize;
use std::collections::BTreeMap;

use crate::error::Error;

#[derive(Deserialize)]
pub struct Config {
    pub state: State,
    pub logging: Logging,
    pub replication: Replication,
}

#[derive(Deserialize)]
pub struct State {
    pub pidfile: String,
    pub statefile: String,
}

#[derive(Deserialize)]
pub struct Logging {
    pub logfile: String,
    pub level: Option<String>,
}

#[derive(Deserialize)]
pub struct Replication {
    pub servers: BTreeMap<String, Server>,
    pub databases: BTreeMap<String, Database>,
    pub synchs: BTreeMap<String, Synch>,
}

#[derive(Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u32,
}

#[derive(Deserialize)]
pub struct Database {
    pub server: String,
    pub name: String,
    pub user: String,
    pub pass: String,
}

#[derive(Deserialize)]
pub struct Synch {
    pub masters: Vec<String>,
    pub slaves: Vec<String>,
}

pub fn parse_config(filename: &str) -> Result<Config, Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // fixme: the following line should work, but it doesn't - using a workaround for now
    // toml::from_str(&contents)?

    let res = toml::from_str(&contents);
    match res {
        Ok(c) => Ok(c),
        Err(e) => Err(From::from(e)),
    }
}
