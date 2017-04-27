use std::env;
use std::path::{Path, PathBuf};

use quote::Tokens;
use walkdir::WalkDir;
use cfg::CargonautsConfig;

const FLAG: &'static str = "used_cargonauts_asset_pipeline";

pub fn assets(cfg: Option<&CargonautsConfig>) -> Tokens {
    let dir = {
        // First, check if they used an asset pipeline
        let used_asset_pipeline = env::args().find(|arg| arg.starts_with(FLAG));
        if let Some(arg) = used_asset_pipeline {
            // Check if they specified an unusual build directory
            let arg =  arg.trim_left_matches(FLAG);
            if arg.len() > 2 && arg.starts_with('[') && arg.ends_with(']') {
                let root: PathBuf = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set").into();
                let range = 1..(arg.len() - 1);
                root.join(&arg[range])
            } else {
                // If they didn't, assets are in OUT_DIR/cargonauts/assets
                let out_dir: PathBuf = env::var("OUT_DIR").expect("OUT_DIR not set").into();
                out_dir.join("cargonauts/assets")
            }
        } else {
            // If they didn't use a pipeline, find their normal assets folder
            let root: PathBuf = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set").into();
            root.join(cfg.and_then(|cfg| cfg.assets()).unwrap_or(Path::new("src/assets")))
        }
    };

    let assets = WalkDir::new(&dir).into_iter().filter_map(|entry| {
        let entry = if let Ok(entry) = entry { entry } else { return None };
        if entry.file_type().is_file() {
            let file_path = entry.path().to_string_lossy();
            let relative_path = entry.path().strip_prefix(&dir).unwrap().to_string_lossy();
            Some(quote!((#relative_path, include_bytes!(#file_path) as &[u8])))
        } else { None }
    }).collect::<Vec<_>>();

    quote! {
        vec!#assets.into_iter().collect();
    }
}
