use std::error::Error;

use serde::Deserialize;
use toml;

use super::utils;

#[derive(Deserialize)]
pub struct DaemonConfig {
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

pub fn parse_cfg_file(cfg_filename: &str) -> Result<DaemonConfig, String> {
    let contents = utils::read_cfg_file(cfg_filename);
    match contents {
        Ok(c) => {
            let config_parse_result = toml::from_str(&c);
            match config_parse_result {
                Ok(config) => Ok(config),
                Err(e) => Err(format!(
                    "Error parsing config file ({}): {}.",
                    &cfg_filename,
                    e.description()
                )),
            }
        }
        Err(e) => Err(format!(
            "Unable to read config file ({}): {}.",
            &cfg_filename,
            e.description()
        )),
    }
}
