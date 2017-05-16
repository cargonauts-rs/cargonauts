use std::fmt::Debug;
use std::io;
use std::rc::Rc;

use futures::{Future, future};

use rigging::Error;
use rigging::resource::ResourceEndpoint;
use rigging::environment::Environment;
use rigging::http;
use rigging::format::*;
use rigging::method::Method;

const MIME: &'static str = "text/plain; charset=utf-8";

#[derive(Default, Clone)]
pub struct SimpleDebug {
    _private: (),
}

impl<T, R, M> Format<T, R, M> for SimpleDebug
where
    T: ResourceEndpoint,
    M: ?Sized + Method<T, Request = ()>,
    R: Debug,
{
    type ReqFuture = future::FutureResult<(), Error>;

    fn receive_request(_: &Rc<Self>, _: http::Request, _: &mut Environment) -> Self::ReqFuture {
        future::ok(())
    }

    fn present_unit(_: &Rc<Self>, future: M::Future, _: TemplateKey, _: &mut Environment) -> http::BoxFuture
        where M: Method<T, Response = ()>
    {
        Box::new(future.then(|result| match result {
            Ok(())  => Ok(http::Response::new().with_status(http::StatusCode::NoContent)),
            Err(e)  => Ok(debug_response(e, http::StatusCode::InternalServerError)),
        }))
    }

    fn present_resource(_: &Rc<Self>, future: M::Future, _: TemplateKey, _: &mut Environment) -> http::BoxFuture
        where M: Method<T, Response = R>, R: ResourceEndpoint
    {
        Box::new(future.then(|result| match result {
            Ok(resource)    => Ok(debug_response(resource, http::StatusCode::Ok)),
            Err(e)          => Ok(debug_response(e, http::StatusCode::InternalServerError)),
        }))
    }

    fn present_collection(_: &Rc<Self>, future: M::Future, _: TemplateKey, _: &mut Environment) -> http::BoxFuture
        where M: Method<T, Response = Vec<R>>, R: ResourceEndpoint
    {
        Box::new(future.then(|result| match result {
            Ok(resources)   => Ok(debug_response(resources, http::StatusCode::Ok)),
            Err(e)          => Ok(debug_response(e, http::StatusCode::InternalServerError)),
        }))
    }

    fn present_error(_: &Rc<Self>, error: Error, _: &mut Environment) -> http::BoxFuture {
        Box::new(future::ok(debug_response(error, http::StatusCode::InternalServerError)))
    }
}

impl BuildFormat for SimpleDebug {
    fn build(_: &[Template]) -> Result<Self, io::Error> {
        Ok(Self { _private: () })
    }
}

fn debug_response<T: Debug>(t: T, status: http::StatusCode) -> http::Response {
    let data = format!("{:?}", t);
    http::Response::new().with_status(status)
                         .with_header(http::headers::ContentLength(data.len() as u64))
                         .with_header(http::headers::ContentType(MIME.parse().unwrap()))
                         .with_body(data)
}
