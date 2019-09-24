use std::collections::BTreeMap;
use std::rc::Rc;

use log::info;

use super::config::ReplicationsConfig;

#[derive(Debug)]
pub struct Replication {
    pub sources: Vec<String>,
    pub targets: Vec<String>,
}

pub fn create_replications(
    config: ReplicationsConfig,
) -> Result<BTreeMap<String, Replication>, String> {
    info!("Validating configured servers...");
    let mut servers: BTreeMap<String, Rc<(String, u32)>> = BTreeMap::new();
    for (name, params) in &config.servers {
        servers.insert(
            name.clone(),
            Rc::from((params.host.clone(), params.port.clone())),
        );
    }
    info!("Done.");

    info!("Validating configured databases...");
    let mut databases: BTreeMap<String, Rc<String>> = BTreeMap::new();
    for (name, params) in &config.databases {
        let server = servers.get(&params.server).unwrap();
        let connection_string =
            build_connection_string(server, &params.name, &params.user, &params.pass);
        databases.insert(name.clone(), Rc::from(connection_string));
    }
    info!("Done.");

    info!("Validating configured replications...");
    let mut replications: BTreeMap<String, Replication> = BTreeMap::new();
    for (name, params) in &config.replications {
        let mut replication = Replication {
            sources: Vec::new(),
            targets: Vec::new(),
        };
        for master in &params.masters {
            let database = databases.get(master).unwrap();
            replication.sources.push(database.to_string());
            replication.targets.push(database.to_string());
        }
        for slave in &params.slaves {
            let database = databases.get(slave).unwrap();
            replication.targets.push(database.to_string());
        }
        replications.insert(name.clone(), replication);
    }
    info!("Done.");

    info!("Replication config parsed successfully.");
    Ok(replications)
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
