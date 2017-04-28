#![feature(associated_consts)]

#[macro_use]
extern crate cargonauts;
extern crate tokio_service;

use cargonauts::api::{Resource, Get, Index, Environment, Error};
use cargonauts::api::GetOne;
use cargonauts::format::Debug;
use cargonauts::futures::{Future, future, Stream, stream};

#[derive(Debug)]
pub struct MyResource { 
    slug: String,
}

relation!(AllCaps => MyResource);

routes! {
    resource MyResource {
        method Get in Debug;
        method Index in Debug;

        relation AllCaps {
            method GetOne in Debug;
        }
    }
}

impl Resource for MyResource {
    type Identifier = String;
}

impl Get for MyResource {
    fn get(slug: String, _: &Environment) -> Box<Future<Item = MyResource, Error = Error>> {
        future::ok(MyResource { slug }).boxed()
    }
}

impl Index for MyResource {
    fn index(_: &Environment) -> Box<Stream<Item = MyResource, Error = Error>> {
        stream::once(Ok(MyResource { slug: String::from("hello-world") })).boxed()
    }
}

impl GetOne<AllCaps> for MyResource {
    fn get_one(slug: String, _: &Environment) -> Box<Future<Item = MyResource, Error = Error>> {
        future::ok(MyResource { slug: slug.to_uppercase() }).boxed()
    }
}

fn main() {
    cargonauts::server::serve(routes).unwrap();
}
