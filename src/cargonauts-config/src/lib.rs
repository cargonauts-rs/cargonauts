#[macro_use]
extern crate serde_derive;
extern crate toml;

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use std::net::SocketAddr;
use std::path::PathBuf;

pub use toml::Value;

#[derive(Deserialize)]
pub struct CargonautsConfig {
    host: Option<SocketAddr>,
    clients: Option<BTreeMap<String, BTreeMap<String, toml::Value>>>,
}

impl CargonautsConfig {
    pub fn find_and_parse() -> Result<CargonautsConfig, Box<Error>> {

        fn find_cargo_toml(mut current: PathBuf) -> io::Result<File> {
            File::open(current.join("Cargo.toml")).or_else(|err| {
                if current.pop() { find_cargo_toml(current) }
                else { Err(err) }
            })
        }

        let file = find_cargo_toml(env::current_dir()?)?;
        CargonautsConfig::from_file(file)
    }

    fn from_file(mut file: File) -> Result<CargonautsConfig, Box<Error>> {
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        CargonautsConfig::from_toml(&buffer).map_err(Into::into)
    }

    fn from_toml(toml: &str) -> Result<CargonautsConfig, toml::de::Error> {
        let cargo: CargoToml = toml::from_str(toml)?;
        Ok(cargo.package.metadata.cargonauts)
    }

    pub fn host(&self) -> Option<SocketAddr> {
        self.host
    }

    pub fn client_cfg(&self, client: &str) -> Option<&BTreeMap<String, Value>> {
        self.clients.as_ref().and_then(|clients| clients.get(client))
    }
}

#[derive(Deserialize)]
struct CargoToml {
    package: Package,
}

#[derive(Deserialize)]
struct Package {
    metadata: Metadata,
}

#[derive(Deserialize)]
struct Metadata {
    cargonauts: CargonautsConfig,
}
