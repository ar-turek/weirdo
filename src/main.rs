extern crate clap;
extern crate log;
extern crate serde;
extern crate simple_logger;
extern crate toml;

mod config;
mod error;
mod logging;
mod replication;

use clap::{App, Arg};
use config::parse_config;
use log::info;
use logging::setup_logging;
use replication::create_synchs;

fn main() {
    let params = get_arg_parser().get_matches();
    let config_filename = params.value_of("config").unwrap();
    let config = match parse_config(config_filename) {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    setup_logging(&config);
    info!("Weirdo starting...");

    let synchs = create_synchs(config).unwrap();
    println!("{:?}", &synchs);
}

fn get_arg_parser<'a, 'b>() -> clap::App<'a, 'b> {
    App::new("weirdo")
        .version("0.1.0")
        .author("Artur Ciesielski <artur.ciesielski@gmail.com>")
        .about("Fast, lightweight and reliable PostgreSQL master-to-master replication tool.")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("config_file")
                .help("Location of the config file")
                .default_value("/etc/weirdo/weirdo.toml"),
        )
        .arg(
            Arg::with_name("--no-daemon")
                .short("-D")
                .long("no-daemon")
                .help("Run weirdo in foreground"),
        )
}
