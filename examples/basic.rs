#![feature(associated_consts)]

#[macro_use]
extern crate cargonauts;

use cargonauts::api::{Resource, Get, Index, Environment, Error};
use cargonauts::format::Debug;
use cargonauts::futures::{Future, BoxFuture, future};
use cargonauts::futures::stream::{self, Stream, BoxStream};

#[derive(Debug)]
struct MyResource { 
    slug: String,
}

impl Resource for MyResource {
    type Identifier = String;
    fn identifier(&self) -> String { self.slug.clone() }
}

impl Get for MyResource {
    fn get(slug: String, _: Environment) -> BoxFuture<MyResource, Error> {
        future::ok(MyResource { slug }).boxed()
    }
}

impl Index for MyResource {
    fn index(_: Environment) -> BoxStream<MyResource, Error> {
        stream::once(Ok(MyResource { slug: String::from("hello-world") })).boxed()
    }
}

routes! {
    resource MyResource {
        method Get in Debug;
        method Index in Debug;
    }
}

fn main() {
    let socket_addr = "127.0.0.1:8000".parse().unwrap();
    cargonauts::server::Http::new().bind(&socket_addr, routes).unwrap().run().unwrap();
}
