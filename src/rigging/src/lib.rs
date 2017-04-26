#![feature(associated_consts)]

extern crate anymap;
extern crate futures;
extern crate hyper;
extern crate tokio_service as tokio;
extern crate tokio_proto as proto;
extern crate tokio_core as core;
extern crate c3po;
extern crate serde;
extern crate tokio_redis as redis;
extern crate url;

pub mod connections;
pub mod endpoint;
pub mod environment;
pub mod format;
pub mod http;
pub mod method;
pub mod request;
pub mod routes;

use std::io;
use std::str::FromStr;

#[derive(Debug)]
pub struct Error;

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Error {
        Error // TODO
    }
}

pub trait Resource: Send + 'static {
    type Identifier: Eq + ToString + FromStr + Send + 'static;
}

pub trait ResourceEndpoint: Resource {
    const ENDPOINT: &'static str;
    const RESOURCE: &'static str;
    const REL_LINKS: &'static [RelationshipLink];
}

pub struct RelationshipLink {
    pub endpoint: &'static str,
    pub relation: &'static str,
}

pub trait Relationship: 'static {
    type Related: Resource;
}

impl<T: Resource> Relationship for T {
    type Related = T;
}

pub trait RelationEndpoint<R>
where
    R: Relationship,
    R::Related: ResourceEndpoint,
    Self: ResourceEndpoint,
{
    const LINK: RelationshipLink;
}

