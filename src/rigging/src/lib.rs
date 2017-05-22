#![feature(associated_consts)]

extern crate anymap;
extern crate backtrace;
extern crate futures;
extern crate hyper;
extern crate tokio_service as tokio;
extern crate tokio_core as core;
extern crate c3po;
extern crate ref_filter_map; // This should 100% be in std!!
extern crate route_recognizer as recognizer;
extern crate serde;
extern crate tokio_redis as redis;
extern crate url;

pub mod connections;
pub mod endpoint;
pub mod environment;
pub mod error;
pub mod format;
pub mod http;
pub mod method;
pub mod resource;
pub mod routes;

pub use error::Error as Error;
