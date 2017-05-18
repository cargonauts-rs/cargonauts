mod document;
mod traits;
mod object;
mod rels;

pub use self::traits::{ApiDeserialize, ClientIdPolicy};

use std::marker::PhantomData;

use futures::{Future, future, Stream};
use serde::de::Deserializer;
use json;

use rigging::Error;
use rigging::resource::{ResourceEndpoint, WithRels};
use rigging::http;

use self::document::DocumentVisitor;
use self::object::Object;
pub trait JsonApiBody<T>: Sized + 'static {
    type Future: Future<Item = Self, Error = Error> + 'static;
    fn parse(body: http::Body) -> Self::Future;
}

impl<T> JsonApiBody<T> for () {
    type Future = future::FutureResult<Self, Error>;
    fn parse(_: http::Body) -> Self::Future {
        future::ok(())
    }
}

impl<T: ResourceEndpoint, P: for<'d> ApiDeserialize<'d>> JsonApiBody<T> for P {
    type Future = Box<Future<Item = Self, Error = Error>>;
    fn parse(body: http::Body) -> Self::Future {
        let future = body.fold(vec![], |mut vec, chunk| -> Result<_, http::Error> {
            vec.extend(&*chunk);
            Ok(vec)
        });
        Box::new(future.then(|result| match result {
            Ok(data)    => {
                let mut deserializer = json::Deserializer::new(json::de::SliceRead::new(&data));
                let visitor: DocumentVisitor<Object<T, P>> = DocumentVisitor(PhantomData);
                match deserializer.deserialize_map(visitor) {
                    Ok(object)  => Ok(object.0),
                    Err(e)      => Err(Error::from_err(e, http::StatusCode::BadRequest))
                }
            }
            Err(e)      => Err(Error::from_err(e, http::StatusCode::InternalServerError)),
        }))
    }
}

impl<T: ResourceEndpoint, P: for<'d> ApiDeserialize<'d>> JsonApiBody<T> for WithRels<T, P> {
    type Future = Box<Future<Item = Self, Error = Error>>;
    fn parse(body: http::Body) -> Self::Future {
        let future = body.fold(vec![], |mut vec, chunk| -> Result<_, http::Error> {
            vec.extend(&*chunk);
            Ok(vec)
        });
        Box::new(future.then(|result| match result {
            Ok(data)    => {
                let mut deserializer = json::Deserializer::new(json::de::SliceRead::new(&data));
                let visitor: DocumentVisitor<Object<T, WithRels<T, P>>> = DocumentVisitor(PhantomData);
                match deserializer.deserialize_map(visitor) {
                    Ok(bridge)  => Ok(bridge.0),
                    Err(e)      => Err(Error::from_err(e, http::StatusCode::BadRequest))
                }
            }
            Err(e)      => Err(Error::from_err(e, http::StatusCode::InternalServerError)),
        }))
    }
}
