#![feature(associated_consts)]

#[macro_use]
extern crate cargonauts;
extern crate tokio_service;

use cargonauts::clients::{Client, Conn, Configure};
use cargonauts::resources::{Resource, Environment, Error};
use cargonauts::methods::Get;
use cargonauts::formats::Debug;
use cargonauts::futures::{Future, future};

use tokio_service::Service;

#[derive(Default)]
struct Foo;

impl<Args> Configure<Args> for Foo {
    type Config = ();
    fn config(_: (), _: Args) -> Foo {
        Foo::default()
    }
}

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

impl Bar {
    fn bar(&self) -> MyResource {
        MyResource { slug: "bar" }
    }
}

impl Client for Bar {
    const CONNECTION_NAME: &'static str = "foo";
    type Connection = Foo;
    fn connect(_: Conn<Self::Connection>) -> Self {
        Bar
    }
}

#[derive(Debug)]
pub struct MyResource {
    slug: &'static str,
}

routes! {
    setup {
        connection to Foo as "foo";
        connection to Foo as "baz";
    }

    resource MyResource {
        method Get in Debug;
    }
}

impl Resource for MyResource {
    type Identifier = String;
}

impl Get for MyResource {
    fn get(slug: String, env: &Environment) -> Box<Future<Item = MyResource, Error = Error>> {
        match &slug[..] {
            "foo"   => Box::new(env.conn_for::<Foo>("foo").and_then(|foo| foo.call(()))),
            "bar"   => Box::new(env.client::<Bar>().map(|bar| bar.bar())),
            "baz"   => Box::new(env.conn_for::<Foo>("baz").and_then(|foo| foo.call(()))),
            _       => future::err(Error).boxed(),
        }
    }
}

fn main() {
    cargonauts::server::serve(routes).unwrap();
}
