use futures::{Future, Stream};

use Error;
use resource::ResourceEndpoint;
use environment::Environment;
use method::Method;
use http;

pub trait Format<T: ResourceEndpoint, M: ?Sized + Method<T>> {
    type ReqFuture: Future<Item = M::Request, Error = Error> + 'static;

    fn receive_request(req: http::Request, env: &mut Environment) -> Self::ReqFuture;

    fn present_unit<F>(future: F, template: Option<Template>, env: &mut Environment) -> http::BoxFuture
        where F: Future<Item = (), Error = Error> + 'static;

    fn present_resource<F>(future: F, template: Option<Template>, env: &mut Environment) -> http::BoxFuture
        where F: Future<Item = M::Response, Error = Error> + 'static;

    fn present_collection<S>(stream: S, template: Option<Template>, env: &mut Environment) -> http::BoxFuture
        where S: Stream<Item = M::Response, Error = Error> + 'static;

    fn present_error(error: Error, env: &mut Environment) -> http::BoxFuture;
}

#[derive(Copy, Clone)]
pub struct Template {
    src: &'static str
}

impl Template {
    #[doc(hidden)]
    pub fn static_prepare(src: &'static str) -> Template {
        Template { src }
    }
}

impl AsRef<str> for Template {
    fn as_ref(&self) -> &str { self.src }
}
