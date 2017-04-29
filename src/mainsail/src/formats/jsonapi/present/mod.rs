mod attributes;
mod traits;
mod document;
mod rels;

use futures::{Future, Stream, future};

use rigging::{ResourceEndpoint, Error};
use rigging::environment::Environment;
use rigging::http;
use rigging::format::{Present, Template};
use rigging::method::Method;

pub use self::traits::{Fields, ApiSerialize};

use self::traits::{Object, ErrorObject};
use self::document::{Document, ErrorDocument};

const MIME: &'static str = "application/vnd.api+json";

impl<T, M> Present<T, M> for super::JsonApi
where
    T: ResourceEndpoint,
    M: ?Sized + Method<T>,
    M::Response: ApiSerialize + ResourceEndpoint,
{
    fn unit<F>(future: F, _: Option<Template>, _: &Environment) -> http::BoxFuture
        where F: Future<Item = (), Error = Error> + 'static,
    {
        Box::new(future.then(move |result| match result {
            Ok(())  => future::ok(http::Response::new().with_status(http::StatusCode::NoContent)),
            Err(e)  => future::ok(error_response(e)),
        }))
    }

    fn resource<F>(future: F, _: Option<Template>, env: &Environment) -> http::BoxFuture
        where F: Future<Item = M::Response, Error = Error> + 'static,
    {
        let fields = Fields::get(env);
        Box::new(future.then(move |result| match result {
            Ok(r)   => {
                let doc = Document {
                    member: Object {
                        inner: &r,
                        fields: fields.as_ref(),
                    }
                };
                let mut buf = vec![];
                match doc.write(&mut buf) {
                    Ok(_)   => future::ok(respond_with(buf, http::StatusCode::Ok)),
                    Err(_)  => panic!(),
                }
            }
            Err(e)  => future::ok(error_response(e)),
        }))
    }

    fn collection<S>(stream: S, _: Option<Template>, env: &Environment) -> http::BoxFuture
        where S: Stream<Item = M::Response, Error = Error> + 'static,
    {
        let fields = Fields::get(env);
        Box::new(stream.collect().then(move |result| match result {
            Ok(r)   => {
                let doc = Document {
                    member: Object {
                        inner: &r,
                        fields: fields.as_ref(),
                    }
                };
                let mut buf = vec![];
                match doc.write(&mut buf) {
                    Ok(_)   => future::ok(respond_with(buf, http::StatusCode::Ok)),
                    Err(_)  => panic!(),
                }
            }
            Err(e)  => future::ok(error_response(e)),
        }))
    }

    fn error(error: Error, _: &Environment) -> http::BoxFuture {
        Box::new(future::ok(error_response(error)))
    }

}

fn error_response(error: Error) -> http::Response {
    let doc = ErrorDocument { error: ErrorObject { error } };
    let mut buf = vec![];
    match doc.serialize(&mut buf) {
        Ok(())  => respond_with(buf, http::StatusCode::InternalServerError),
        Err(_)  => panic!()
    }
}

fn respond_with(data: Vec<u8>, status: http::StatusCode) -> http::Response {
    http::Response::new().with_status(status)
                   .with_header(http::headers::ContentLength(data.len() as u64))
                   .with_header(http::headers::ContentType(MIME.parse().unwrap()))
                   .with_body(data)
}
