#![feature(associated_consts)]

extern crate anymap;
extern crate futures;
extern crate hyper;
extern crate tokio_service as tokio;
extern crate tokio_proto as proto;
extern crate tokio_core as core;
extern crate c3po;

pub mod connections;
pub mod endpoint;
pub mod environment;
pub mod format;
pub mod http;
pub mod request;
pub mod routes;

use std::str::FromStr;

#[derive(Debug)]
pub struct Error;

pub trait Resource: Send + 'static {
    type Identifier: Eq + ToString + FromStr + Send + 'static;
    fn identifier(&self) -> Self::Identifier;
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

use request::Request;

pub trait Method<T: Resource> {
    const ROUTE: routes::Route<'static>;

    type Request: Request<T>;
    type Response: Resource;
    type Outcome: 'static;

    fn call(req: Self::Request, env: environment::Environment) -> Self::Outcome;
}
