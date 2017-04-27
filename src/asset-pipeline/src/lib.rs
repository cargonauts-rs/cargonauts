extern crate walkdir;
extern crate cargonauts_config;

use std::error::Error;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use cargonauts_config::CargonautsConfig;

pub fn build_assets<F>(f: F) -> Result<(), Box<Error>>
where
    F: FnOnce(walkdir::Iter, &Path) -> Result<(), Box<Error>>
{
    let cfg = CargonautsConfig::find_and_parse().ok();
    let asset_path = {
            let root: PathBuf = env::var("CARGO_MANIFEST_DIR")?.into();
            let sub = cfg.as_ref().and_then(|cfg| cfg.assets()).unwrap_or(Path::new("src/assets"));
            root.join(sub)
    };
    let output_path = {
        let out_dir: PathBuf = env::var("OUT_DIR")?.into();
        out_dir.join("cargonauts/assets")
    };
    fs::create_dir_all(&output_path)?;

    println!("cargo:rustc-cfg=cargonauts_asset_pipeline");

    let iter = walkdir::WalkDir::new(asset_path).into_iter();
    f(iter, output_path.as_path())
}
