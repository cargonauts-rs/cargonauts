#![feature(associated_consts)]

#[macro_use]
extern crate cargonauts;
#[macro_use]
extern crate jsonapi_derive;
extern crate tokio_service;

use cargonauts::resources::{Resource, Environment, Error};
use cargonauts::methods::{Get, Index, Post, GetOne};
use cargonauts::formats::JsonApi;
use cargonauts::futures::{Future, future};

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
        Box::new(future::ok(MyResource { slug }))
    }
}

impl Index for MyResource {
    fn index(_: &Environment) -> Box<Future<Item = Vec<MyResource>, Error = Error>> {
        Box::new(future::ok(vec![MyResource { slug: String::from("hello-world") }]))
    }
}

#[derive(ApiDeserialize)]
pub struct MyResourcePost {
    attrs: i32,
}

impl Post for MyResource {
    type Post = MyResourcePost;
    fn post(_: Self::Post, _: &Environment) -> Box<Future<Item = MyResource, Error = Error>> {
        Box::new(future::ok(MyResource { slug: String::from("hello-world") }))
    }
}

impl GetOne<AllCaps> for MyResource {
    fn get_one(slug: String, _: &Environment) -> Box<Future<Item = MyResource, Error = Error>> {
        Box::new(future::ok(MyResource { slug: slug.to_uppercase() }))
    }
}

fn main() {
    cargonauts::server::serve(routes).unwrap();
}

routes! {
    resource MyResource {
        method Get, Index in JsonApi;

        has one AllCaps {
            method GetOne in JsonApi;
        }
    }
}
