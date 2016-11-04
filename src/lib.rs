#![feature(specialization)]

extern crate itertools;
extern crate serde;
extern crate serde_json as json;

pub mod api;
pub mod presenter;
pub mod repr;
pub mod router;

#[macro_use]
mod macros;
pub mod _internal;

pub use serde::{Deserialize, Deserializer, Serialize, SerializeTo, Serializer};
pub use json::{Value, from_value, to_value};

#[cfg(test)]
mod tests;
