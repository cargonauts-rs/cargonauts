use std::env;
use std::path::PathBuf;

use quote::Tokens;
use walkdir::WalkDir;
use cfg::CargonautsConfig;

use ast::Routes;

const FLAG: &'static str = "used_cargonauts_asset_pipeline";

pub fn assets(cfg: &CargonautsConfig, routes: &Routes) -> Tokens {
    let handler = asset_handler(routes);

    let dir = {
        // First, check if they used an asset pipeline
        let used_asset_pipeline = env::args().find(|arg| arg.starts_with(FLAG));
        if let Some(arg) = used_asset_pipeline {
            // Check if they specified an unusual build directory
            let arg =  arg.trim_left_matches(FLAG);
            if arg.len() > 2 && arg.starts_with('[') && arg.ends_with(']') {
                let range = 1..(arg.len() - 1);
                cfg.project_root().join(&arg[range])
            } else {
                // If they didn't, assets are in OUT_DIR/cargonauts/assets
                let out_dir: PathBuf = env::var("OUT_DIR").expect("OUT_DIR not set").into();
                out_dir.join("cargonauts/assets")
            }
        } else {
            // If they didn't use a pipeline, find their normal assets folder
            cfg.assets()
        }
    };

    let assets = WalkDir::new(&dir).into_iter().filter_map(|entry| entry.ok().and_then(|entry| {
        if entry.file_type().is_file() {
            let path = entry.path();
            let file_path = path.to_string_lossy();
            let url_path = if entry.file_name().to_string_lossy().starts_with(cfg.index()) {
                path.strip_prefix(&dir).unwrap().parent().unwrap().to_string_lossy()
            } else {
                path.strip_prefix(&dir).unwrap().to_string_lossy()
            };
            Some(quote!({
                let service = ::cargonauts::routing::AssetHandler {
                    handler: #handler,
                    data: include_bytes!(#file_path),
                };
                routes.add(::cargonauts::server::Method::Get, String::from(#url_path), Box::new(service) as ::cargonauts::routing::Handler);
            }))
        } else { None }
    })).collect::<Vec<_>>();

    quote! {
        #(#assets)*
    }
}

fn asset_handler(routes: &Routes) -> Tokens {
    if let Some(handler) = routes.asset_handler.as_ref() {
        quote!(#handler)
    } else {
        quote!(::cargonauts::routing::default_asset_handler)
    }
}
