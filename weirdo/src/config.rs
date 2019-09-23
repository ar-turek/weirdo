use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;

use toml;
use serde::Deserialize;


#[derive(Deserialize)]
pub struct Config {
    pub state: StateConfig,
    pub logging: LoggingConfig,
}

#[derive(Deserialize)]
pub struct StateConfig {
    pub pidfile: String,
    pub statefile: String,
}

#[derive(Deserialize)]
pub struct LoggingConfig {
    pub logfile: String,
    pub level: Option<String>,
}


pub fn parse_cfg_file(cfg_file: &str) -> Result<Config, String> {
    let contents = read_cfg_file(cfg_file);
    match contents {
        Ok(c) => {
            let config_parse_result = toml::from_str(&c);
            match config_parse_result {
                Ok(config) => Ok(config),
                Err(e) => Err(format!("Error parsing config file ({}): {}.", &cfg_file, e.description())),
            }
        },
        Err(e) => Err(format!("Unable to read config file ({}): {}.", &cfg_file, e.description())),
    }
}

fn read_cfg_file(cfg_file: &str) -> io::Result<String> {
    let mut file = File::open(cfg_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}
