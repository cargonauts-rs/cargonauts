mod collection;
mod relation;
mod resource;

use self::collection::CollectionDocument;
use self::relation::RelationshipDocument;
use self::resource::ResourceDocument;

use std::io::Read;
use std::marker::PhantomData;
use io_adapter::ReadAdapter;

use api::Error;
use api::raw::{RawUpdate, RawHasPatch, RawReceived, Relationship};
use api::rel::Relation;
use serde::de::{self, Visitor, MapVisitor};
use Deserialize;
use Deserializer;
use receiver::{Receiver, PatchReceiver};

pub struct JsonApi<D: Deserializer + ReadAdapter<R>, R: Read> {
    deserializer: D,
    _spoopy: PhantomData<R>,
}

impl<D, R> ReadAdapter<R> for JsonApi<D, R>
where
    D: Deserializer + ReadAdapter<R>,
    R: Read,
{
    fn wrap(read: R) -> Self {
        JsonApi {
            deserializer: D::wrap(read),
            _spoopy: PhantomData,
        }
    }

    fn into_inner(self) -> R {
        self.deserializer.into_inner()
    }
}

impl<T, R, D> Receiver<T, R> for JsonApi<D, R>
where
    T: RawUpdate + Deserialize,
    R: Read,
    D: Deserializer + ReadAdapter<R>,
{
    fn receive_resource(mut self) -> Result<RawReceived<T, T>, Error> {
        ResourceDocument::deserialize(&mut self.deserializer).map(|x| x.0).or(Err(Error::BadRequest))
    }

    fn receive_collection(mut self) -> Result<Vec<RawReceived<T, T>>, Error> {
        CollectionDocument::deserialize(&mut self.deserializer).map(|x| x.0).or(Err(Error::BadRequest))
    }

    fn receive_rel<Rel: Relation>(mut self) -> Result<Relationship, Error> {
        RelationshipDocument::<Rel>::deserialize(&mut self.deserializer).map(|x| x.0).or(Err(Error::BadRequest))
    }
}

impl<T, R, D, X> PatchReceiver<T, R, X> for JsonApi<D, R>
where
    T: RawHasPatch<X>,
    T::Patch: Deserialize,
    R: Read,
    D: Deserializer + ReadAdapter<R>,
{
    fn receive_patch(mut self) -> Result<RawReceived<T, T::Patch>, Error> {
        ResourceDocument::deserialize(&mut self.deserializer).map(|x| x.0).or(Err(Error::BadRequest))
    }
}

struct ObjectVisitor<T: Deserialize>(PhantomData<T>);

impl<T: Deserialize> Visitor for ObjectVisitor<T> {
    type Value = T;

    fn visit_map<V: MapVisitor>(&mut self, mut visitor: V) -> Result<T, V::Error> {
        if let Some(key) = visitor.visit_key::<String>()? {
            match &key[..] {
                "data"      => visitor.visit_value(),
                otherwise   => return Err(de::Error::invalid_value(otherwise)),
            }
        } else { visitor.missing_field("data") }
    }
}
