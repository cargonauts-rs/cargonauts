#![feature(associated_consts)]

#[macro_use] extern crate cargonauts;

use cargonauts::formats::Debug;
use cargonauts::methods::Get;
use cargonauts::{Resource, Environment, Error};
use cargonauts::futures::{Future, future};

#[derive(Debug)]
pub struct Echo {
    echo: String,
}

impl Resource for Echo {
    type Identifier = String;
}

impl Get for Echo {
    fn get(echo: String, _: Environment) -> Box<Future<Item = Echo, Error = Error>> {
        future::ok(Echo { echo }).boxed()
    }
}

routes! {
    resource Echo {
        method Get in Debug;
    }
}

fn main() {
    cargonauts::server::serve(routes).unwrap();
}
