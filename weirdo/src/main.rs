extern crate clap;
extern crate serde;
extern crate toml;

mod config;
mod logging;
mod replications;

use config::{parse_cfg_file, parse_replications_file};
use logging::setup_logging;
use clap::{App, Arg};


fn main() {
    let params = get_arg_parser().get_matches();
    let config = parse_cfg_file(params.value_of("config").unwrap()).expect("");
    let replications = parse_replications_file(params.value_of("replications").unwrap()).expect("");

    setup_logging(&config.logging);

    println!("{:?}", config);
    println!("{:?}", replications);
}

fn get_arg_parser<'a, 'b>() -> clap::App<'a, 'b> {
    App::new("weirdo")
        .version("0.1.0")
        .author("Artur Ciesielski <artur.ciesielski@gmail.com>")
        .about("Fast, lightweight and reliable PostgreSQL master-to-master replication tool.")
        .arg(Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("config_file")
                .help("Config file location")
                .default_value("/etc/weirdo/weirdo.toml"))
        .arg(Arg::with_name("replications")
                .short("r")
                .long("replications")
                .value_name("replications_file")
                .help("Replications file location")
                .default_value("/etc/weirdo/replications.toml"))
        .arg(Arg::with_name("--no-daemon")
                .short("-D")
                .long("no-daemon")
                .help("Run weirdo in foreground"))
}
