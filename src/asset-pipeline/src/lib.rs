extern crate cargonauts_config;

use std::error::Error;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use cargonauts_config::CargonautsConfig;

pub fn build_assets<F>(f: F) -> Result<(), Box<Error>>
where
    F: FnOnce(&Path, &Path) -> Result<(), Box<Error>>
{
    let cfg = CargonautsConfig::find_and_parse()?;
    let output_path = {
        let out_dir: PathBuf = env::var("OUT_DIR")?.into();
        out_dir.join("cargonauts/assets")
    };
    fs::create_dir_all(&output_path)?;

    println!("cargo:rustc-cfg=used_cargonauts_asset_pipeline");

    f(&cfg.assets(), &output_path)
}
