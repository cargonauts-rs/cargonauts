use std::marker::PhantomData;

use serde::de::{Deserialize, Deserializer};
use json;

use rigging::Error;
use rigging::resource::{ResourceEndpoint, WithRels};

use super::document::DocumentVisitor;

pub trait ApiDeserialize<'d>: ResourceEndpoint + Sized + 'static {
    fn deserialize<D: Deserializer<'d>>(deserializer: D) -> Result<Self, D::Error>;
    fn deserialize_with_rels<D: Deserializer<'d>>(deserializer: D) -> Result<WithRels<Self>, D::Error>;
}

pub struct Bridge<T>(T);

impl<'d, T: ApiDeserialize<'d>> Deserialize<'d> for Bridge<T> {
    fn deserialize<D: Deserializer<'d>>(deserializer: D) -> Result<Self, D::Error> {
        T::deserialize(deserializer).map(Bridge)
    }
}

impl<'d, T: ApiDeserialize<'d>> Bridge<T> {
    pub fn parse(data: &'d [u8]) -> Result<T, Error> {
        let mut deserializer = json::Deserializer::new(json::de::SliceRead::new(data));
        let visitor: DocumentVisitor<Self> = DocumentVisitor(PhantomData);
        match deserializer.deserialize_map(visitor) {
            Ok(bridge)  => Ok(bridge.0),
            Err(_)      => Err(Error)
        }
    }
}

pub struct RelBridge<T: ResourceEndpoint>(WithRels<T>);

impl<'d, T: ApiDeserialize<'d> + ResourceEndpoint> Deserialize<'d> for RelBridge<T> {
    fn deserialize<D: Deserializer<'d>>(deserializer: D) -> Result<Self, D::Error> {
        T::deserialize_with_rels(deserializer).map(RelBridge)
    }
}

impl<'d, T: ApiDeserialize<'d> + ResourceEndpoint> RelBridge<T> {
    pub fn parse(data: &'d [u8]) -> Result<WithRels<T>, Error> {
        let mut deserializer = json::Deserializer::new(json::de::SliceRead::new(data));
        let visitor: DocumentVisitor<Self> = DocumentVisitor(PhantomData);
        match deserializer.deserialize_map(visitor) {
            Ok(bridge)  => Ok(bridge.0),
            Err(_)      => Err(Error)
        }
    }
}
