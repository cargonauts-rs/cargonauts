use std::fmt::Debug;

use futures::{Future, Stream, future};

use rigging::{ResourceEndpoint, Error};
use rigging::environment::Environment;
use rigging::http;
use rigging::format::*;
use rigging::method::Method;
use rigging::request::Request;

const MIME: &'static str = "ext/plain; charset=utf-8";

#[derive(Default, Clone)]
pub struct SimpleDebug {
    _private: (),
}

impl<T, M> Format<T, M> for SimpleDebug
where
    T: ResourceEndpoint,
    M: ?Sized + Method<T>,
    M::Request: Request<T, BodyParts = ()>,
    M::Response: Debug,
{
    type Receiver = super::BasicReceiver;
    type Presenter = Self;
}

impl<T, M> Present<T, M> for SimpleDebug
where
    T: ResourceEndpoint,
    M: ?Sized + Method<T>,
    M::Response: Debug,
{
    fn unit<F>(future: F, _: Option<Template>, _: &Environment) -> http::BoxFuture
        where F: Future<Item = (), Error = Error> + 'static,
    {
        Box::new(future.then(|result| match result {
            Ok(())  => Ok(http::Response::new().with_status(http::StatusCode::NoContent)),
            Err(e)  => Ok(debug_response(e, http::StatusCode::InternalServerError)),
        }))
    }

    fn resource<F>(future: F, _: Option<Template>, _: &Environment) -> http::BoxFuture
        where F: Future<Item = M::Response, Error = Error> + 'static,
    {
        Box::new(future.then(|result| match result {
            Ok(resource)    => Ok(debug_response(resource, http::StatusCode::Ok)),
            Err(e)          => Ok(debug_response(e, http::StatusCode::InternalServerError)),
        }))
    }

    fn collection<S>(stream: S, _: Option<Template>, _: &Environment) -> http::BoxFuture
        where S: Stream<Item = M::Response, Error = Error> + 'static,
    {
        Box::new(stream.collect().then(|result| match result {
            Ok(resources)   => Ok(debug_response(resources, http::StatusCode::Ok)),
            Err(e)          => Ok(debug_response(e, http::StatusCode::InternalServerError)),
        }))
    }

    fn error(error: Error, _: &Environment) -> http::BoxFuture {
        Box::new(future::ok(debug_response(error, http::StatusCode::InternalServerError)))
    }
}

fn debug_response<T: Debug>(t: T, status: http::StatusCode) -> http::Response {
    let data = format!("{:?}", t);
    http::Response::new().with_status(status)
                         .with_header(http::headers::ContentLength(data.len() as u64))
                         .with_header(http::headers::ContentType(MIME.parse().unwrap()))
                         .with_body(data)
}
