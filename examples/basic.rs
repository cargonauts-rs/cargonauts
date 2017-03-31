#![feature(associated_consts)]

#[macro_use]
extern crate cargonauts;

use cargonauts::api::{Resource, Get, Index, Environment, Error};
use cargonauts::format::Debug;
use cargonauts::futures::{Future, BoxFuture, future};
use cargonauts::futures::stream::{self, Stream, BoxStream};

#[derive(Debug)]
struct Something(u32);

impl Resource for Something {
    type Identifier = u32;
    fn identifier(&self) -> u32 { self.0 }
}

impl Get for Something {
    fn get(id: u32, _: Environment) -> BoxFuture<Something, Error> {
        future::ok(Something(id)).boxed()
    }
}

impl Index for Something {
    fn index(_: Environment) -> BoxStream<Something, Error> {
        stream::once(Ok(Something(0))).boxed()
    }
}

routes! {
    #[format(Debug)]
    resource Something: Get + Index;
}

fn main() {
    let socket_addr = "127.0.0.1:8000".parse().unwrap();
    cargonauts::server::Http::new().bind(&socket_addr, routes).unwrap().run().unwrap();
}
