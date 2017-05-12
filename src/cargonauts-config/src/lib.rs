#[macro_use]
extern crate serde_derive;
extern crate toml;

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};

pub use toml::Value;

#[derive(Deserialize)]
pub struct CargonautsConfig {
    #[serde(default = "project_root")]
    project_root: PathBuf,
    host: Option<SocketAddr>,
    templates: Option<PathBuf>,
    assets: Option<PathBuf>,
    conns: Option<BTreeMap<String, BTreeMap<String, toml::Value>>>,
    env: Option<Env>,
}

impl Default for CargonautsConfig {
    fn default() -> CargonautsConfig {
        CargonautsConfig {
            project_root: project_root(),
            host: None,
            templates: None,
            assets: None,
            conns: None,
            env: None,
        }
    }
}

impl CargonautsConfig {
    pub fn find_and_parse() -> Result<CargonautsConfig, Box<Error>> {
        let path: PathBuf = env::var("CARGO_MANIFEST_DIR")?.into();
        let file = File::open(path.join("Cargo.toml"))?;
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

    pub fn project_root(&self) -> &Path {
        &self.project_root
    }

    pub fn host(&self) -> SocketAddr {
        self.host.unwrap_or_else(|| "127.0.0.1:7878".parse().unwrap())
    }

    pub fn templates(&self) -> PathBuf {
        if let Some(ref templates) = self.templates {
            self.project_root.join(templates)
        } else {
            self.project_root.join("src/templates")
        }
    }

    pub fn assets(&self) -> PathBuf {
        if let Some(ref assets) = self.assets {
            self.project_root.join(assets)
        } else {
            self.project_root.join("src/assets")
        }
    }

    pub fn conn_cfg(&self, conn: &str) -> Option<&BTreeMap<String, Value>> {
        self.conns.as_ref().and_then(|conns| conns.get(conn))
    }

    pub fn env(&self, profile: &str) -> Option<&BTreeMap<String, String>> {
        match profile {
            "dev"       => self.env.as_ref().and_then(|env| env.dev.as_ref()),
            "release"   => self.env.as_ref().and_then(|env| env.release.as_ref()),
            "test"      => self.env.as_ref().and_then(|env| env.test.as_ref()),
            _           => None,
        }
    }
}

#[derive(Deserialize)]
pub struct Env {
    dev: Option<BTreeMap<String, String>>,
    release: Option<BTreeMap<String, String>>,
    test: Option<BTreeMap<String, String>>
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
    #[serde(default = "CargonautsConfig::default")]
    cargonauts: CargonautsConfig,
}

fn project_root() -> PathBuf {
    env::var("CARGO_MANIFEST_DIR").unwrap().into()
}
