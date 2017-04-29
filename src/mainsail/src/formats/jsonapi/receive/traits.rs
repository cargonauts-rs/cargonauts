use std::marker::PhantomData;

use serde::de::{Deserialize, Deserializer};
use json;

use rigging::Error;
use rigging::resource::{ResourceEndpoint, WithRels};

use super::document::DocumentVisitor;

pub trait ApiDeserialize<'d>: ResourceEndpoint + Sized + 'static {
    fn deserialize<D: Deserializer<'d>>(deserializer: D) -> Result<WithRels<Self>, D::Error>;
}

pub struct Bridge<T: ResourceEndpoint>(WithRels<T>);

impl<'d, T: ApiDeserialize<'d> + ResourceEndpoint> Deserialize<'d> for Bridge<T> {
    fn deserialize<D: Deserializer<'d>>(deserializer: D) -> Result<Self, D::Error> {
        T::deserialize(deserializer).map(Bridge)
    }
}

impl<'d, T: ApiDeserialize<'d> + ResourceEndpoint> Bridge<T> {
    pub fn parse(data: &'d [u8]) -> Result<WithRels<T>, Error> {
        let mut deserializer = json::Deserializer::new(json::de::SliceRead::new(data));
        match deserializer.deserialize_map(DocumentVisitor(PhantomData)) {
            Ok(bridge)  => Ok(bridge.0),
            Err(_)      => Err(Error)
        }
    }
}
