extern crate clap;
extern crate serde;
extern crate toml;

mod config;
mod logging;

use config::parse_cfg_file;
use logging::setup_logging;
use clap::{App, Arg};


fn main() {
    let params = get_arg_parser().get_matches();
    let config = parse_cfg_file(params.value_of("config").unwrap()).expect("");

    setup_logging(&config.logging);
}

fn get_arg_parser<'a, 'b>() -> clap::App<'a, 'b> {
    App::new("weirdo")
        .version("0.1.0")
        .author("Artur Ciesielski <artur.ciesielski@gmail.com>")
        .about("Fast, lightweight and reliable PostgreSQL master-to-master replication tool.")
        .arg(Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("cfg_file")
                .help("Override default config file location")
                .default_value("/etc/weidro.cfg"))
}
