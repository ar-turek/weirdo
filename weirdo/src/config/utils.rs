use std::fs::File;
use std::io;
use std::io::Read;


pub fn read_cfg_file(cfg_file: &str) -> io::Result<String> {
    let mut file = File::open(cfg_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}
