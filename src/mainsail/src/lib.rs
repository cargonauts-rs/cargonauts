#![feature(associated_consts)]

extern crate handlebars as hbs;
extern crate futures;
extern crate rigging;
extern crate serde;
extern crate serde_json as json;
extern crate serde_urlencoded as urlencoded;
extern crate url;

#[macro_use] extern crate serde_derive;

pub mod methods;
pub mod formats;
