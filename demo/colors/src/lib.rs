#![feature(associated_consts)]

#[macro_use] extern crate cargonauts;
#[macro_use] extern crate serde_derive;

extern crate rand;
extern crate serde;

mod clients;
mod formats;
mod methods;
mod middleware;
mod resources;
mod routing;

pub use routing::routes;
