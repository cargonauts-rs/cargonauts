use std::borrow::Borrow;
use std::collections::HashMap;
use std::io;
use std::rc::Rc;

use tokio::{Service, NewService};
use futures::{Future, future};

use http;
use environment::Environment;

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct RouteKey {
    key: RouteKeyRef<'static>,
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct RouteKeyRef<'a> {
    endpoint: &'a str,
    route: Route<'a>,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Route<'a> {
    pub method: http::Method,
    pub kind: Kind<'a>,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Kind<'a> {
    Resource,
    Collection,
    Relationship(&'a str),
}

impl RouteKey {
    pub fn new(endpoint: &'static str, route: Route<'static>) -> RouteKey {
        RouteKey { key: RouteKeyRef { endpoint, route } }
    }
}

impl<'a> Borrow<RouteKeyRef<'a>> for RouteKey {
    fn borrow(&self) -> &RouteKeyRef<'a> {
        &self.key
    }
}

impl<'a> RouteKeyRef<'a> {
    fn req(req: &'a http::Request) -> Option<RouteKeyRef<'a>> {
        let mut path_components = req.path().split('/').filter(|p| !p.is_empty());
        path_components.next().map(|endpoint| {
            let kind = match path_components.next() {
                Some(_) => {
                    match path_components.next() {
                        Some(rel)   => Kind::Relationship(rel),
                        None        => Kind::Resource,
                    }
                }
                None    => Kind::Collection,
            };
            let method = req.method().clone();
            RouteKeyRef {
                endpoint,
                route: Route {
                    method,
                    kind,
                }
            }
        })
    }
}

#[derive(Clone)]
pub struct RoutingTable {
    routes: Rc<HashMap<RouteKey, Handler>>,
    env: Environment,
}

impl RoutingTable {
    pub fn new(routes: HashMap<RouteKey, Handler>, env: Environment) -> RoutingTable {
        RoutingTable { routes: Rc::new(routes), env }
    }
}

impl Service for RoutingTable {
    type Request = http::Request;
    type Response = http::Response;
    type Error = http::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        match RouteKeyRef::req(&req).and_then(|route| self.routes.get(&route)) {
            Some(handle)    => handle(req, self.env.clone()),
            None            => not_found(),
        }
    }
}

impl NewService for RoutingTable {
    type Request = http::Request;
    type Response = http::Response;
    type Error = http::Error;
    type Instance = Self;
    type Future = future::FutureResult<Self, io::Error>;

    fn new_service(&self) -> Self::Future {
        future::ok(self.clone())
    }
}

pub type Handler = fn(http::Request, Environment) -> Box<Future<Item = http::Response, Error = http::Error>>;

pub fn not_found() -> Box<Future<Item = http::Response, Error = http::Error>> {
    future::ok(http::Response::new().with_status(http::StatusCode::NotFound)).boxed()
}
