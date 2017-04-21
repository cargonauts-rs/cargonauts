#![feature(associated_consts)]

#[macro_use]
extern crate cargonauts;
extern crate tokio_service;

#[derive(Default)]
struct Foo;

impl tokio_service::NewService for Foo {
    type Request = ();
    type Response = ();
    type Error = ();
    type Instance = Self;
    type Future = cargonauts::futures::future::FutureResult<Self, ::std::io::Error>;
    fn new_service(&self) -> Self::Future {
        ::cargonauts::futures::future::ok(Foo)
    }
}

impl tokio_service::Service for Foo {
    type Request = ();
    type Response = ();
    type Error = ();
    type Future = cargonauts::futures::future::FutureResult<(), ()>;
    fn call(&self, _: ()) -> Self::Future {
        ::cargonauts::futures::future::ok(())
    }
}

use cargonauts::api::{Resource, Get, Index, Environment, Error};
use cargonauts::api::relations::GetOne;
use cargonauts::format::Debug;
use cargonauts::futures::{Future, future, Stream, stream};

#[derive(Debug)]
pub struct MyResource { 
    slug: String,
}

relation!(AllCaps => MyResource);

routes! {
    setup {
        client for Foo;
    }

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
    fn identifier(&self) -> String { self.slug.clone() }
}

impl Get for MyResource {
    fn get(slug: String, _: Environment) -> Box<Future<Item = MyResource, Error = Error>> {
        future::ok(MyResource { slug }).boxed()
    }
}

impl Index for MyResource {
    fn index(_: Environment) -> Box<Stream<Item = MyResource, Error = Error>> {
        stream::once(Ok(MyResource { slug: String::from("hello-world") })).boxed()
    }
}

impl GetOne<AllCaps> for MyResource {
    fn get_one(slug: String, _: Environment) -> Box<Future<Item = MyResource, Error = Error>> {
        future::ok(MyResource { slug: slug.to_uppercase() }).boxed()
    }
}

fn main() {
    cargonauts::server::serve(routes).unwrap();
}
