extern crate cargonauts;
extern crate todo;

use std::panic;
use std::process::Command;

use cargonauts::config::CargonautsConfig;

fn main() {
    if cfg!(feature = "proxy") {
        let cfg = CargonautsConfig::find_and_parse().unwrap();
        let host = format!("http://{}", cfg.host());
        println!("{}", host);
        let mut child = Command::new("ember").arg("server")
                                .current_dir(cfg.assets())
                                .spawn().unwrap();

        let _ = panic::catch_unwind(|| {
            cargonauts::serve(todo::routes).unwrap();
        });

        child.kill().unwrap();
    } else {
        cargonauts::serve(todo::routes).unwrap();
    }

}
