use log::Level;

use crate::config::Config;

pub fn setup_logging(_config: &Config) {
    simple_logger::init_with_level(Level::Info).unwrap();
}
