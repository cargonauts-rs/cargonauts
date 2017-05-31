use std::io;
use std::rc::Rc;

use hbs;
use serde::{Deserialize, Serialize};
use futures::{Future, future, Stream};
use urlencoded::Deserializer;
use url::form_urlencoded;

use rigging::Error;
use rigging::http;
use rigging::environment::Environment;
use rigging::format::*;
use rigging::method::Method;
use rigging::resource::ResourceEndpoint;

const MIME: &'static str = "text/html; charset=utf-8";

/// Render your response using a handlebars template.
///
/// This format is for server-side rendering of HTML. The template for each
/// endpoint can be found at `src/templates/$resource/$method.html.hbs`.
///
/// Request bodies will be parsed as form encoded data; that is, this format
/// is designed to receive HTML form data.
///
/// Like all formats, it imposes some constraints on the method and the
/// resource that it is paired with:
///
/// * The **resource** type must implement `Serialize`, which will be used by
/// the handlebars rendering engine.
/// * The request type of the **method** must implement `Deserialize`; it will
/// be deserialized from the urlencoded format.
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
    M: ?Sized + Method<T>,
    M::Request: for<'d> Deserialize<'d> + 'static,
    R: Serialize,
{
    type ReqFuture = Box<Future<Item = M::Request, Error = Error>>;

    fn receive_request(_: &Rc<Self>, req: http::Request, _: &Environment) -> Self::ReqFuture {
        let future = req.body().fold(vec![], |mut vec, chunk| -> Result<_, http::Error> {
            vec.extend(&*chunk);
            Ok(vec)
        });
        Box::new(future.then(|result| match result {
            Ok(data)    => {
                let deserializer = Deserializer::new(form_urlencoded::parse(&data));
                M::Request::deserialize(deserializer).map_err(|e| Error::from_err(e, http::StatusCode::BadRequest))
            }
            Err(e)      => Err(Error::from_err(e, http::StatusCode::InternalServerError)),
        }))
    }

    fn present_unit(this: &Rc<Self>, future: M::Future, key: TemplateKey, _: &Environment) -> http::BoxFuture
    where
        M: Method<T, Response = ()>
    {
        let this = this.clone();
        Box::new(future.then(move |result| {
            match result.and_then(|_| this.registry.render(&key.to_string(), &()).map_err(|e| {
                Error::from_err(e, http::StatusCode::BadRequest)
            })) {
                Ok(body)    => Ok(respond_with(body)),
                Err(err)    => Ok(respond_with_err(err)),
            }
        }))
    }

    fn present_resource(this: &Rc<Self>, future: M::Future, key: TemplateKey, _: &Environment) -> http::BoxFuture
    where M: Method<T, Response = R>,
        R: ResourceEndpoint
    {
        let this = this.clone();
        Box::new(future.then(move |result| {
            match result.and_then(|resource| this.registry.render(&key.to_string(), &Object { resource }).map_err(|e| {
                Error::from_err(e, http::StatusCode::BadRequest)
            })) {
                Ok(body)    => Ok(respond_with(body)),
                Err(err)    => Ok(respond_with_err(err)),
            }
        }))
    }

    fn present_collection(this: &Rc<Self>, future: M::Future, key: TemplateKey, _: &Environment) -> http::BoxFuture
    where
        M: Method<T, Response = Vec<R>>,
        R: ResourceEndpoint
    {
        let this = this.clone();
        Box::new(future.then(move |result| {
            match result.and_then(|resources| this.registry.render(&key.to_string(), &Objects { resources }).map_err(|e| {
                Error::from_err(e, http::StatusCode::BadRequest)
            })) {
                Ok(body)    => Ok(respond_with(body)),
                Err(err)    => Ok(respond_with_err(err)),
            }
        }))
    }

    fn present_error(_: &Rc<Self>, error: Error, _: &Environment) -> http::BoxFuture {
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

fn respond_with_err(err: Error) -> http::Response {
    #[cfg(debug_assertions)]
    let body = format!("{:?}", err);
    #[cfg(not(debug_assertions))]
    let body = format!("{}", err);
    http::Response::new().with_status(err.status_code())
                         .with_header(http::headers::ContentLength(body.len() as u64))
                         .with_body(body)
}
