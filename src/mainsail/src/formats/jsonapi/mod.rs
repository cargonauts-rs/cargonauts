mod present;
mod receive;

mod fieldset;

use futures::{Future, Stream, future};

use rigging::Error;
use rigging::resource::ResourceEndpoint;
use rigging::environment::Environment;
use rigging::format::{Format, Template};
use rigging::http;
use rigging::method::Method;

pub use self::fieldset::Fields;
pub use self::present::ApiSerialize;
pub use self::receive::{ApiDeserialize, ClientIdPolicy};

use self::present::*;
use self::receive::*;

pub struct JsonApi;

const MIME: &'static str = "application/vnd.api+json";

impl<T, M, P> Format<T, M> for JsonApi
where
    T: ResourceEndpoint,
    M: ?Sized + Method<T, Request = P>,
    M::Response: ApiSerialize + ResourceEndpoint,
    P: JsonApiBody<T>,
{
    type ReqFuture = P::Future;

    fn receive_request(req: http::Request, env: &mut Environment) -> Self::ReqFuture {
        if let Some(fields) = Fields::<M::Response>::new(&req) {
            env.store(fields);
        }
        P::parse(req.body())
    }

    fn present_unit<F>(future: F, _: Option<Template>, _: &mut Environment) -> http::BoxFuture
        where F: Future<Item = (), Error = Error> + 'static,
    {
        Box::new(future.then(move |result| match result {
            Ok(())  => future::ok(http::Response::new().with_status(http::StatusCode::NoContent)),
            Err(e)  => future::ok(error_response(e)),
        }))
    }

    fn present_resource<F>(future: F, _: Option<Template>, env: &mut Environment) -> http::BoxFuture
        where F: Future<Item = M::Response, Error = Error> + 'static,
    {
        let fields = env.take::<Fields<M::Response>>();
        Box::new(future.then(move |result| match result {
            Ok(r)   => {
                let doc = Document {
                    member: Object {
                        inner: &r,
                        fields: fields.as_ref(),
                    }
                };
                let buf = vec![];
                match doc.write(buf) {
                    Ok(buf) => future::ok(respond_with(buf, http::StatusCode::Ok)),
                    Err(e)  => panic!("{:?}", e),
                }
            }
            Err(e)  => future::ok(error_response(e)),
        }))
    }

    fn present_collection<S>(stream: S, _: Option<Template>, env: &mut Environment) -> http::BoxFuture
        where S: Stream<Item = M::Response, Error = Error> + 'static,
    {
        let fields = env.take::<Fields<M::Response>>();
        Box::new(stream.collect().then(move |result| match result {
            Ok(r)   => {
                let doc = Document {
                    member: Object {
                        inner: &r,
                        fields: fields.as_ref(),
                    }
                };
                let buf = vec![];
                match doc.write(buf) {
                    Ok(buf) => future::ok(respond_with(buf, http::StatusCode::Ok)),
                    Err(_)  => panic!(),
                }
            }
            Err(e)  => future::ok(error_response(e)),
        }))
    }

    fn present_error(error: Error, _: &mut Environment) -> http::BoxFuture {
        Box::new(future::ok(error_response(error)))
    }
}

fn error_response(error: Error) -> http::Response {
    let doc = ErrorDocument { error: ErrorObject { error } };
    let buf = vec![];
    match doc.write(buf) {
        Ok(buf) => respond_with(buf, http::StatusCode::InternalServerError),
        Err(_)  => panic!()
    }
}

fn respond_with(data: Vec<u8>, status: http::StatusCode) -> http::Response {
    http::Response::new().with_status(status)
                   .with_header(http::headers::ContentLength(data.len() as u64))
                   .with_header(http::headers::ContentType(MIME.parse().unwrap()))
                   .with_body(data)
}
