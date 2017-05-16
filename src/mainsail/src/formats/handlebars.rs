use std::io;
use std::rc::Rc;

use hbs;
use serde::Serialize;
use futures::{Future, future};

use rigging::Error;
use rigging::http;
use rigging::environment::Environment;
use rigging::format::*;
use rigging::method::Method;
use rigging::resource::ResourceEndpoint;

const MIME: &'static str = "text/plain; charset=utf-8";

pub struct Handlebars {
    registry: hbs::Handlebars,
}

#[derive(Serialize)]
struct Object<T: Serialize> {
    resource: T,
}

#[derive(Serialize)]
struct Objects<T: Serialize> {
    resources: T,
}

impl<T, R, M> Format<T, R, M> for Handlebars
where
    T: ResourceEndpoint,
    M: ?Sized + Method<T, Request = ()>,
    R: Serialize,
{
    type ReqFuture = future::FutureResult<(), Error>;

    fn receive_request(_: &Rc<Self>, _: http::Request, _: &mut Environment) -> Self::ReqFuture {
        future::ok(())
    }

    fn present_unit(this: &Rc<Self>, future: M::Future, key: TemplateKey, _: &mut Environment) -> http::BoxFuture
    where
        M: Method<T, Response = ()>
    {
        let this = this.clone();
        Box::new(future.then(move |result| {
            match result.and_then(|_| this.registry.render(&key.to_string(), &()).map_err(|_| Error)) {
                Ok(body)    => Ok(respond_with(body)),
                Err(err)    => Ok(respond_with_err(err)),
            }
        }))
    }

    fn present_resource(this: &Rc<Self>, future: M::Future, key: TemplateKey, _: &mut Environment) -> http::BoxFuture
    where M: Method<T, Response = R>,
        R: ResourceEndpoint
    {
        let this = this.clone();
        Box::new(future.then(move |result| {
            match result.and_then(|resource| this.registry.render(&key.to_string(), &Object { resource }).map_err(|_| Error)) {
                Ok(body)    => Ok(respond_with(body)),
                Err(err)    => Ok(respond_with_err(err)),
            }
        }))
    }

    fn present_collection(this: &Rc<Self>, future: M::Future, key: TemplateKey, _: &mut Environment) -> http::BoxFuture
    where
        M: Method<T, Response = Vec<R>>,
        R: ResourceEndpoint
    {
        let this = this.clone();
        Box::new(future.then(move |result| {
            match result.and_then(|resources| this.registry.render(&key.to_string(), &Objects { resources }).map_err(|_| Error)) {
                Ok(body)    => Ok(respond_with(body)),
                Err(err)    => Ok(respond_with_err(err)),
            }
        }))
    }

    fn present_error(_: &Rc<Self>, error: Error, _: &mut Environment) -> http::BoxFuture {
        Box::new(future::ok(respond_with_err(error)))
    }
}

impl BuildFormat for Handlebars {
    fn build(templates: &[Template]) -> Result<Self, io::Error> {
        let mut hbs = Handlebars { registry: hbs::Handlebars::new() };
        for template in templates {
            hbs.registry.register_template_string(&template.key.to_string(), template.template).map_err(|_| -> io::Error { unimplemented!() })?;
        }
        Ok(hbs)
    }
}

fn respond_with(body: String) -> http::Response {
    let status = if body.len() == 0 { http::StatusCode::NoContent } else { http::StatusCode::Ok };
    http::Response::new().with_status(status)
                         .with_header(http::headers::ContentLength(body.len() as u64))
                         .with_header(http::headers::ContentType(MIME.parse().unwrap()))
                         .with_body(body)
}

fn respond_with_err(_: Error) -> http::Response {
    http::Response::new().with_status(http::StatusCode::InternalServerError)
}
