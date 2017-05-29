#![feature(associated_consts)]

#[macro_use] extern crate cargonauts;

use cargonauts::formats::Debug;
use cargonauts::methods::{Get, GetOne};
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

relation!(AllCaps => Echo);

impl GetOne<AllCaps> for Echo {
    fn get_one(echo: String, _: Environment) -> Box<Future<Item = Echo, Error = Error>> {
        future::ok(Echo { echo: echo.to_uppercase() }).boxed()
    }
}

routes! {
    resource Echo {
        method Get in Debug;

        has one AllCaps {
            method GetOne in Debug;
        }
    }
}

fn main() {
    cargonauts::server::serve(routes).unwrap();
}
