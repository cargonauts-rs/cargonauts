use futures::{Future};

use Error;
use resource::ResourceEndpoint;
use environment::Environment;
use method::Method;
use http;

pub trait Format<T: ResourceEndpoint, R, M: ?Sized + Method<T>> {
    type ReqFuture: Future<Item = M::Request, Error = Error> + 'static;

    fn receive_request(req: http::Request, env: &mut Environment) -> Self::ReqFuture;

    fn present_unit(future: M::Future, template: Option<Template>, env: &mut Environment) -> http::BoxFuture
        where M: Method<T, Response = ()>;

    fn present_resource(future: M::Future, template: Option<Template>, env: &mut Environment) -> http::BoxFuture
        where M: Method<T, Response = R>, R: ResourceEndpoint;

    fn present_collection(future: M::Future, template: Option<Template>, env: &mut Environment) -> http::BoxFuture
        where M: Method<T, Response = Vec<R>>, R: ResourceEndpoint;

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
