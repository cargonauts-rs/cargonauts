#![feature(associated_consts, conservative_impl_trait)]

#[macro_use] extern crate cargonauts;
#[macro_use] extern crate jsonapi_derive;
#[macro_use] extern crate serde_derive;

extern crate chrono;
extern crate uuid;
extern crate serde;
extern crate serde_json as json;

mod clients;
mod middleware;
mod resources;
mod routing;

pub use routing::routes;
