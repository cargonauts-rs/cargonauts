#![feature(specialization)]

extern crate itertools;
extern crate serde;
extern crate serde_json as json;

pub mod api;
pub mod presenter;
pub mod router;

#[macro_use]
mod macros;
mod links;
pub mod _internal;

pub use serde::{Deserialize, Deserializer, Serialize, SerializeTo, Serializer};
pub use json::{Value, from_value, to_value};

#[cfg(test)]
mod tests;

// TODO figure out how to have a user defined const
const BASE_URL: &'static str = "https://example.org/api";
