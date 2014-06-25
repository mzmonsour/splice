extern crate toml = "rust-toml";

use std::io::net::ip::SocketAddr;
use std::from_str::FromStr;

// TODO: Add a Windows implementation of this function
#[cfg(unix)]
pub fn default_path() -> Option<Path> {
    match ::std::os::homedir() {
        Some(mut v) => { v.push(".config/splice/splice.conf"); Some(v) },
        None => None,
    }
}

pub fn load_default() -> Option<Config> {
    match default_path() {
        Some(v) => load(&v),
        None => None,
    }
}

pub fn load(loc: &Path) -> Option<Config> {
    let toml = match toml::parse_from_path(loc) {
        Ok(v) => v,
        Err(e) => return None
    };
    Some(Config {
        default_server_addr:
            match toml.lookup("default_address").and_then(|a| a.get_str()) {
                 Some(v) => v.clone(),
                 None => return None,
            },
        default_server_port:
            match toml.lookup("default_port").and_then(|a| a.get_int()) {
                Some(v) => v as u16,
                None => return None,
            },
    })
}

pub struct Config {
    pub default_server_addr: String,
    pub default_server_port: u16,
}
