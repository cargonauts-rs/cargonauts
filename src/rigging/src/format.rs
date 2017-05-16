use std::fmt;
use std::io;
use std::rc::Rc;

use anymap::AnyMap;
use futures::Future;

use Error;
use resource::ResourceEndpoint;
use environment::Environment;
use method::Method;
use http;

pub trait Format<T: ResourceEndpoint, R, M: ?Sized + Method<T>>: BuildFormat {
    type ReqFuture: Future<Item = M::Request, Error = Error> + 'static;

    fn receive_request(this: &Rc<Self>, req: http::Request, env: &mut Environment) -> Self::ReqFuture;

    fn present_unit(this: &Rc<Self>, future: M::Future, key: TemplateKey, env: &mut Environment) -> http::BoxFuture
        where M: Method<T, Response = ()>;

    fn present_resource(this: &Rc<Self>, future: M::Future, key: TemplateKey, env: &mut Environment) -> http::BoxFuture
        where M: Method<T, Response = R>, R: ResourceEndpoint;

    fn present_collection(this: &Rc<Self>, future: M::Future, key: TemplateKey, env: &mut Environment) -> http::BoxFuture
        where M: Method<T, Response = Vec<R>>, R: ResourceEndpoint;

    fn present_error(this: &Rc<Self>, error: Error, env: &mut Environment) -> http::BoxFuture;
}

pub trait BuildFormat: Sized + 'static {
    fn build(templates: &[Template]) -> Result<Self, io::Error>;
}

pub struct Template {
    pub key: TemplateKey,
    pub template: &'static str,
}

#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone)]
pub struct TemplateKey {
    method: &'static str,
    resource: &'static str,
    rel: Option<&'static str>,
}

impl TemplateKey {
    #[doc(hidden)]
    pub fn new(method: &'static str, resource: &'static str) -> Self {
        Self { method, resource, rel: None }
    }

    #[doc(hidden)]
    pub fn new_rel(method: &'static str, rel: &'static str, resource: &'static str) -> Self {
        Self { method, resource, rel: Some(rel) }
    }
}

impl fmt::Display for TemplateKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.rel {
            Some(rel)   => write!(f, "{}/{}/{}", self.resource, rel, self.method),
            None        => write!(f, "{}/{}", self.resource, self.method),
        }
    }
}

pub struct FormatLender {
    map: AnyMap,
}

impl Default for FormatLender {
    fn default() -> Self {
        Self { map: AnyMap::new() }
    }
}

impl FormatLender {
    pub fn store<F: BuildFormat>(&mut self, format: F) {
        self.map.insert(Rc::new(format));
    }

    pub fn get<F: BuildFormat>(&self) -> Rc<F> {
        self.map.get::<Rc<F>>().unwrap().clone()
    }
}
