extern crate asset_pipeline;

use std::process::Command;

fn main() {
    asset_pipeline::build_assets(|src, dst| {
        // Only build ember if not in a 'proxy' build
        if !cfg!(feature = "proxy") {
            Command::new("ember").arg("build").arg("--prod")
                    .current_dir(src)
                    .arg("-o").arg(dst)
                    .output()?;
        }

        Ok(())
    }).unwrap();
}
