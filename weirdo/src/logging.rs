use super::config::DaemonConfig;

pub fn setup_logging(_config: &DaemonConfig) {
    simple_logger::init().unwrap();
}
