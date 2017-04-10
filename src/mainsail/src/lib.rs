#![feature(associated_consts)]

extern crate anymap;
extern crate futures;

pub mod relations;

use std::sync::{Arc, Mutex};
use std::str::FromStr;

use futures::BoxFuture;
use futures::stream::BoxStream;

pub use anymap::AnyMap;

pub trait ResourceEndpoint: Resource {
    const ENDPOINT: &'static str;
    const RESOURCE: &'static str;
    const REL_LINKS: &'static [relations::RelationshipLink];
}

pub trait Resource: Send + 'static {
    type Identifier: FromStr + ToString + Send + 'static;
    fn identifier(&self) -> Self::Identifier;
}

pub type Environment = Arc<Mutex<AnyMap>>;

#[derive(Debug)]
pub struct Error;

pub trait Get: Resource {
    fn get(identifier: Self::Identifier, env: Environment) -> BoxFuture<Self, Error>
    where Self: Sized;
}

pub trait Index: Resource {
    fn index(env: Environment) -> BoxStream<Self, Error>
    where Self: Sized;
}
