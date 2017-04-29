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
pub mod resource;
pub mod routes;

pub use self::resource::*;

use std::io;

#[derive(Debug)]
pub struct Error;

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Error {
        Error // TODO
    }
}

