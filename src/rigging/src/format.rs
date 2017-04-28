use futures::{Future, Stream};

use {ResourceEndpoint, Error};
use environment::Environment;
use method::Method;
use request::Request;
use http;

pub trait Format<T: ResourceEndpoint, M: ?Sized + Method<T>> {
    type Receiver: Receive<T, M::Request>;
    type Presenter: Present<T, M>;
}

pub trait Receive<T: ResourceEndpoint, R: Request<T>> {
    fn receive(req: http::Request, env: &Environment) -> Result<R::BodyParts, Error>;
}

pub trait Present<T: ResourceEndpoint, M: ?Sized + Method<T>>: Send + 'static {
    fn unit<F>(future: F, template: Option<Template>, env: &Environment) -> http::BoxFuture
        where F: Future<Item = (), Error = Error> + 'static;

    fn resource<F>(future: F, template: Option<Template>, env: &Environment) -> http::BoxFuture
        where F: Future<Item = M::Response, Error = Error> + 'static;

    fn collection<S>(stream: S, template: Option<Template>, env: &Environment) -> http::BoxFuture
        where S: Stream<Item = M::Response, Error = Error> + 'static;

    fn error(error: Error, env: &Environment) -> http::BoxFuture;
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
