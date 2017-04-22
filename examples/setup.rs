#![feature(associated_consts)]

#[macro_use]
extern crate cargonauts;
extern crate tokio_service;

use cargonauts::api::{Resource, Get, Environment, Error};
use cargonauts::format::Debug;
use cargonauts::futures::{Future, future};

use tokio_service::Service;

#[derive(Default)]
struct Foo;

impl tokio_service::NewService for Foo {
    type Request = ();
    type Response = MyResource;
    type Error = Error;
    type Instance = Self;
    type Future = cargonauts::futures::future::FutureResult<Self, ::std::io::Error>;
    fn new_service(&self) -> Self::Future {
        future::ok(Foo)
    }
}

impl Service for Foo {
    type Request = ();
    type Response = MyResource;
    type Error = Error;
    type Future = cargonauts::futures::future::FutureResult<MyResource, Error>;
    fn call(&self, _: ()) -> Self::Future {
        future::ok(MyResource { slug: "foo" })
    }
}

#[derive(Default)]
struct Bar;

impl tokio_service::NewService for Bar {
    type Request = ();
    type Response = MyResource;
    type Error = Error;
    type Instance = Self;
    type Future = cargonauts::futures::future::FutureResult<Self, ::std::io::Error>;
    fn new_service(&self) -> Self::Future {
        future::ok(Bar)
    }
}

impl Service for Bar {
    type Request = ();
    type Response = MyResource;
    type Error = Error;
    type Future = cargonauts::futures::future::FutureResult<MyResource, Error>;
    fn call(&self, _: ()) -> Self::Future {
        future::ok(MyResource { slug: "bar" })
    }
}

#[derive(Debug)]
pub struct MyResource {
    slug: &'static str,
}

routes! {
    setup {
        client for Foo;
        client for Bar;
    }

    resource MyResource {
        method Get in Debug;
    }
}

impl Resource for MyResource {
    type Identifier = String;
    fn identifier(&self) -> String { self.slug.to_owned() }
}

impl Get for MyResource {
    fn get(slug: String, env: Environment) -> Box<Future<Item = MyResource, Error = Error>> {
        match &slug[..] {
            "foo"   => Box::new(env.conn::<Foo>().and_then(|foo| foo.call(()))),
            "bar"   => Box::new(env.conn::<Bar>().and_then(|bar| bar.call(()))),
            _       => future::err(Error).boxed(),
        }
    }
}

fn main() {
    cargonauts::server::serve(routes).unwrap();
}
