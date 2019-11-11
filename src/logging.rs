use log::LevelFilter;
use std::str::FromStr;

use crate::config::Config;

pub fn setup_logging(config: &Config) {
    let level = match &config.logging.level {
        Some(ref l) => l,
        None => "info",
    };
    let level_filter = LevelFilter::from_str(level).unwrap_or(LevelFilter::Info);
    simple_logging::log_to_file(&config.logging.logfile, level_filter)
        .expect("Failed to initialize logging, refusing to proceed.");
}
