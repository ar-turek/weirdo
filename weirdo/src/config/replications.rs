use std::collections::BTreeMap;
use std::error::Error;

use toml;
use serde::Deserialize;

use super::utils;


#[derive(Deserialize, Debug)]
pub struct ReplicationsConfig {
    pub servers: BTreeMap<String, Server>,
    pub databases: BTreeMap<String, Database>,
    pub synchros: BTreeMap<String, Synchro>,
}

#[derive(Deserialize, Debug)]
pub struct Server {
    pub host: String,
    pub port: u32,
}

#[derive(Deserialize, Debug)]
pub struct Database {
    pub server: String,
    pub name: String,
    pub user: String,
    pub pass: String,
}

#[derive(Deserialize, Debug)]
pub struct Synchro {
    pub masters: Vec<String>,
    pub slaves: Vec<String>,
}


pub fn parse_replications_file(replications_filename: &str) -> Result<ReplicationsConfig, String> {
    let contents = utils::read_cfg_file(replications_filename);
    match contents {
        Ok(c) => {
            let config_parse_result = toml::from_str(&c);
            match config_parse_result {
                Ok(config) => Ok(config),
                Err(e) => Err(format!("Error parsing config file ({}): {}.", &replications_filename, e.description())),
            }
        },
        Err(e) => Err(format!("Unable to read config file ({}): {}.", &replications_filename, e.description())),
    }
}
