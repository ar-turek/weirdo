use std::collections::BTreeMap;
use std::rc::Rc;

use log::info;
use postgres::{Connection, TlsMode};

use crate::config::Config;

#[derive(Debug)]
pub struct Replication {
    pub sources: Vec<Rc<Connection>>,
    pub targets: Vec<Rc<Connection>>,
}

pub fn create_synchs(config: Config) -> Result<BTreeMap<String, Replication>, String> {
    let mut servers: BTreeMap<String, Rc<(String, u32)>> = BTreeMap::new();
    for (name, params) in &config.replication.servers {
        servers.insert(
            name.clone(),
            Rc::from((params.host.clone(), params.port.clone())),
        );
    }
    info!("Validated configured servers.");

    let mut databases: BTreeMap<String, String> = BTreeMap::new();
    for (name, params) in &config.replication.databases {
        let server = servers.get(&params.server).unwrap();
        let connection_string =
            build_connection_string(server, &params.name, &params.user, &params.pass);
        databases.insert(name.clone(), connection_string);
    }
    info!("Validated configured databases.");

    let mut synchs: BTreeMap<String, Replication> = BTreeMap::new();
    for (name, params) in &config.replication.synchs {
        let mut replication = Replication {
            sources: Vec::new(),
            targets: Vec::new(),
        };
        for master in &params.masters {
            let database = databases.get(master).unwrap();
            let connection =
                Rc::from(Connection::connect(database.as_str(), TlsMode::None).unwrap());
            replication.sources.push(connection.clone());
            replication.targets.push(connection.clone());
        }
        for slave in &params.slaves {
            let database = databases.get(slave).unwrap();
            let connection =
                Rc::from(Connection::connect(database.as_str(), TlsMode::None).unwrap());
            replication.targets.push(connection.clone());
        }
        synchs.insert(name.clone(), replication);
    }
    info!("Validated configured replications.");

    info!("Replication config parsed successfully.");
    Ok(synchs)
}

fn build_connection_string(
    server: &(String, u32),
    name: &String,
    user: &String,
    pass: &String,
) -> String {
    format!(
        "postgres://{}:{}@{}:{}/{}",
        user, pass, server.0, server.1, name
    )
}
