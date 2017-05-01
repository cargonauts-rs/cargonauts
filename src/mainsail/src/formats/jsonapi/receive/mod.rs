mod document;
mod traits;
mod object;

pub use self::traits::{ApiDeserialize, ClientIdPolicy};

use std::marker::PhantomData;

use futures::{Future, future, Stream};
use serde::de::Deserializer;
use json;

use rigging::Error;
use rigging::resource::{ResourceEndpoint, WithRels};
use rigging::environment::Environment;
use rigging::format::Receive;
use rigging::http;
use rigging::request::Request;

use self::document::DocumentVisitor;
use self::object::Object;

impl<T, R, P> Receive<T, R> for super::JsonApi
where
    T: ResourceEndpoint,
    R: Request<T, BodyParts = P>,
    P: JsonApiBody,
{
    type Future = P::Future;
    fn receive(req: http::Request, _: &Environment) -> Self::Future {
        // TODO set env
        P::parse(req.body())
    }
}

pub trait JsonApiBody: Sized + 'static {
    type Future: Future<Item = Self, Error = Error> + 'static;
    fn parse(body: http::Body) -> Self::Future;
}

impl JsonApiBody for () {
    type Future = future::FutureResult<Self, Error>;
    fn parse(_: http::Body) -> Self::Future {
        future::ok(())
    }
}

impl<T: ResourceEndpoint + for<'d> ApiDeserialize<'d>> JsonApiBody for T {
    type Future = Box<Future<Item = Self, Error = Error>>;
    fn parse(body: http::Body) -> Self::Future {
        let future = body.fold(vec![], |mut vec, chunk| -> Result<_, http::Error> {
            vec.extend(&*chunk);
            Ok(vec)
        });
        Box::new(future.then(|result| match result {
            Ok(data)    => {
                let mut deserializer = json::Deserializer::new(json::de::SliceRead::new(&data));
                let visitor: DocumentVisitor<Object<T>> = DocumentVisitor(PhantomData);
                match deserializer.deserialize_map(visitor) {
                    Ok(object)  => Ok(object.0),
                    Err(_)      => Err(Error)
                }
            }
            Err(_)      => Err(Error),
        }))
    }
}

impl<T: ResourceEndpoint + for<'d> ApiDeserialize<'d>> JsonApiBody for WithRels<T> {
    type Future = Box<Future<Item = Self, Error = Error>>;
    fn parse(body: http::Body) -> Self::Future {
        let future = body.fold(vec![], |mut vec, chunk| -> Result<_, http::Error> {
            vec.extend(&*chunk);
            Ok(vec)
        });
        Box::new(future.then(|result| match result {
            Ok(data)    => {
                let mut deserializer = json::Deserializer::new(json::de::SliceRead::new(&data));
                let visitor: DocumentVisitor<Object<WithRels<T>>> = DocumentVisitor(PhantomData);
                match deserializer.deserialize_map(visitor) {
                    Ok(bridge)  => Ok(bridge.0),
                    Err(_)      => Err(Error)
                }
            }
            Err(_)      => Err(Error),
        }))
    }
}
