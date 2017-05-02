#![feature(associated_consts)]

#[macro_use]
extern crate cargonauts;
#[macro_use]
extern crate jsonapi_derive;
extern crate tokio_service;

use cargonauts::api::{Resource, Post, Get, Index, Environment, Error};
use cargonauts::api::GetOne;
use cargonauts::format::JsonApi;
use cargonauts::futures::{Future, future, Stream, stream};

#[derive(ApiSerialize)]
pub struct MyResource { 
    #[api_id]
    slug: String,
}

relation!(AllCaps => MyResource);

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

#[derive(ApiDeserialize)]
pub struct MyResourcePost {
    attrs: i32,
}

impl Post for MyResource {
    type Post = MyResourcePost;
    fn post(_: Self::Post, _: &Environment) -> Box<Future<Item = MyResource, Error = Error>> {
        future::ok(MyResource { slug: String::from("hello-world") }).boxed()
    }
}

impl GetOne<AllCaps> for MyResource {
    fn get_one(slug: String, _: &Environment) -> Box<Future<Item = MyResource, Error = Error>> {
        future::ok(MyResource { slug: slug.to_uppercase() }).boxed()
    }
}

fn asrt<T: ::cargonauts::format::jsonapi::JsonApiBody<MyResource>>() { }

fn main() {
    asrt::<MyResourcePost>();
    cargonauts::server::serve(routes).unwrap();
}

routes! {
    resource MyResource {
        method Get in JsonApi;
        method Index in JsonApi;

        relation AllCaps {
            method GetOne in JsonApi;
        }
    }
}
