mod collection;
mod identifier;
mod post;
mod relation;
mod resource;

use self::collection::CollectionDocument;
use self::identifier::IdentifiersDocument;
use self::post::PostDocument;
use self::relation::{ToOneDocument, ToManyDocument};
use self::resource::ResourceDocument;

use std::io::Read;
use std::marker::PhantomData;
use io_adapter::ReadAdapter;

use api::Error;
use api::raw::{RawResource, RawHasPatch, RawReceived, Identifier};
use api::rel::{ToOne, ToMany};
use serde::de::{self, Visitor, MapVisitor};
use Deserialize;
use Deserializer;
use receiver::{Receiver, PatchReceiver, Post};
use router::Request;

pub struct JsonApi<D: Deserializer, R: Read> {
    _spoopy: PhantomData<(D, R)>,
}

impl<T, R, D> Receiver<T, R> for JsonApi<D, R>
where
    T: RawResource + Deserialize,
    R: Request,
    D: Deserializer + ReadAdapter<R>,
{
    fn receive_post(request: R) -> Result<Post<T>, Error> {
        let mut deserializer = D::wrap(request);
        PostDocument::deserialize(&mut deserializer).map(|x| x.0).or(Err(Error::BadRequest))
    }

    fn receive_resource(request: R) -> Result<RawReceived<T, T>, Error> {
        let mut deserializer = D::wrap(request);
        ResourceDocument::deserialize(&mut deserializer).map(|x| x.0).or(Err(Error::BadRequest))
    }

    fn receive_collection(request: R) -> Result<Vec<RawReceived<T, T>>, Error> {
        let mut deserializer = D::wrap(request);
        CollectionDocument::deserialize(&mut deserializer).map(|x| x.0).or(Err(Error::BadRequest))
    }

    fn receive_to_one<Rel: ToOne>(request: R) -> Result<Option<Identifier>, Error> {
        let mut deserializer = D::wrap(request);
        ToOneDocument::<Rel>::deserialize(&mut deserializer).map(|x| x.0).or(Err(Error::BadRequest))
    }
    fn receive_to_many<Rel: ToMany>(request: R) -> Result<Vec<Identifier>, Error> {
        let mut deserializer = D::wrap(request);
        ToManyDocument::<Rel>::deserialize(&mut deserializer).map(|x| x.0).or(Err(Error::BadRequest))
    }

    fn receive_identifiers(request: R) -> Result<Vec<Identifier>, Error> {
        let mut deserializer = D::wrap(request);
        IdentifiersDocument::<T>::deserialize(&mut deserializer).map(|x| x.0).or(Err(Error::BadRequest))
    }
}

impl<T, R, D, X> PatchReceiver<T, R, X> for JsonApi<D, R>
where
    T: RawHasPatch<X>,
    T::Patch: Deserialize,
    R: Request,
    D: Deserializer + ReadAdapter<R>,
{
    fn receive_patch(request: R) -> Result<RawReceived<T, T::Patch>, Error> {
        let mut deserializer = D::wrap(request);
        ResourceDocument::deserialize(&mut deserializer).map(|x| x.0).or(Err(Error::BadRequest))
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
