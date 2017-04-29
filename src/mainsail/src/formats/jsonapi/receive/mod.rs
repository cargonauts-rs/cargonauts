mod document;
mod traits;

pub use self::traits::ApiDeserialize;


use futures::{Future, future, Stream};

use rigging::Error;
use rigging::resource::{ResourceEndpoint, WithRels};
use rigging::environment::Environment;
use rigging::format::Receive;
use rigging::http;
use rigging::request::Request;

use self::traits::Bridge;

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

impl<T: ResourceEndpoint + for<'d> ApiDeserialize<'d>> ParseBody for WithRels<T> {
    type Future = Box<Future<Item = Self, Error = Error>>;
    fn parse(body: http::Body) -> Self::Future {
        let future = body.fold(vec![], |mut vec, chunk| -> Result<_, http::Error> {
            vec.extend(&*chunk);
            Ok(vec)
        });
        Box::new(future.then(|result| match result {
            Ok(data)    => Bridge::parse(&data),
            Err(_)      => Err(Error),
        }))
    }
}
