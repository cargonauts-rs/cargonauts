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
        future::ok(MyResource { slug }).boxed()
    }
}

impl Index for MyResource {
    fn index(_: &Environment) -> Box<Future<Item = Vec<MyResource>, Error = Error>> {
        future::ok(vec![MyResource { slug: String::from("hello-world") }]).boxed()
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
