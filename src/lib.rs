#![feature(specialization)]

extern crate io_adapter;
extern crate futures;
extern crate itertools;
extern crate serde;
extern crate serde_json;

pub mod json {
    pub use serde_json::{Serializer, Deserializer};

    #[cfg(test)]
    pub use serde_json::{Value, to_value};
}

pub mod api;
pub mod presenter;
pub mod receiver;
pub mod repr;
pub mod router;

#[macro_use]
mod macros;
pub mod _internal;

pub use io_adapter::{ReadAdapter, WriteAdapter};
pub use futures::{IntoFuture, Future};
pub use serde::{Deserialize, Deserializer, Serialize, SerializeTo, Serializer};

#[cfg(test)]
mod tests;
