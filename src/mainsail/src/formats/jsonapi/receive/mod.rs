mod document;

use std::marker::PhantomData;

use futures::{Future, future, Stream};
use serde::de::{Deserialize, Deserializer};
use json;

use rigging::{Error, ResourceEndpoint, RelationEndpoint, Relationship, Resource};
use rigging::environment::Environment;
use rigging::format::Receive;
use rigging::http;
use rigging::request::Request;

use self::document::DocumentVisitor;

impl<T, R, P> Receive<T, R> for super::JsonApi
where
    T: ResourceEndpoint,
    R: Request<T, BodyParts = P>,
    P: ParseBody,
{
    type Future = P::Future;
    fn receive(req: http::Request, _: &Environment) -> Self::Future {
        // TODO set env
        P::parse(req.body())
    }
}

pub trait ParseBody: Sized + 'static {
    type Future: Future<Item = Self, Error = Error> + 'static;
    fn parse(body: http::Body) -> Self::Future;
}

impl ParseBody for () {
    type Future = future::FutureResult<Self, Error>;
    fn parse(_: http::Body) -> future::FutureResult<Self, Error> {
        future::ok(())
    }
}

impl<P: for<'d> ApiDeserialize<'d> + HasRelations> ParseBody for Object<P> {
    type Future = Box<Future<Item = Self, Error = Error>>;
    fn parse(body: http::Body) -> Self::Future {
        let future = body.fold(vec![], |mut vec, chunk| -> Result<_, http::Error> {
            vec.extend(&*chunk);
            Ok(vec)
        });
        Box::new(future.then(|result| match result {
            Ok(data)    => Object::parse(&data),
            Err(_)      => Err(Error),
        }))
    }
}

pub trait ApiDeserialize<'d>: HasRelations + Sized + 'static {
    fn deserialize<D: Deserializer<'d>>(deserializer: D) -> Result<Object<Self>, D::Error>;
}

pub trait HasRelations {
    type Relations: RelationUpdate;
}

pub trait RelationUpdate {
    fn find_id<R>(&self) -> Option<<R::Related as Resource>::Identifier>
    where
        Self: RelationEndpoint<R>,
        R: Relationship,
        R::Related: ResourceEndpoint;
}

pub struct Object<P: HasRelations> {
    pub attrs: P,
    relationships: P::Relations,
}

impl<P: HasRelations> Object<P> {
    pub fn rel_id<R>(&self) -> Option<<R::Related as Resource>::Identifier>
    where
        P::Relations: RelationEndpoint<R>,
        R: Relationship,
        R::Related: ResourceEndpoint
    {
        self.relationships.find_id::<R>()
    }
}

impl<'d, P: ApiDeserialize<'d> + HasRelations> Object<P> {
    fn parse(data: &'d [u8]) -> Result<Object<P>, Error> {
        let mut deserializer = json::Deserializer::new(json::de::SliceRead::new(data));
        deserializer.deserialize_map(DocumentVisitor(PhantomData)).map_err(|_| Error)
    }
}

impl<'d, P: ApiDeserialize<'d> + HasRelations> Deserialize<'d> for Object<P> {
    fn deserialize<D: Deserializer<'d>>(deserializer: D) -> Result<Self, D::Error> {
        P::deserialize(deserializer)
    }
}
