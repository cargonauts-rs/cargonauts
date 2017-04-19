use std::collections::HashMap;

use tokio::Service;
use futures::{Future, BoxFuture, future};

use http;

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Route {
    pub method: http::Method,
    pub kind: Kind<'static>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Kind<'a> {
    Resource,
    Collection,
    Relationship(&'a str),
}

pub struct Routes {
    pub endpoints: Vec<(&'static str, Vec<Route>)>,
}

impl Routes {
    fn routes_for(&self, resource: &str) -> Option<&[Route]> {
        let idx = self.endpoints.binary_search_by_key(&resource, |&(e, _)| e).ok();
        idx.map(|idx| &self.endpoints[idx].1[..])
    }

    pub fn process(&self, req: &http::Request) -> Option<Route> {
        let mut path_components = req.path().split('/').filter(|p| !p.is_empty());
        let routes = path_components.next().and_then(|resource| self.routes_for(resource));
        routes.and_then(|routes| {
            let mut routes = routes.iter().filter(|route| &route.method == req.method());
            let route = match path_components.next() {
                Some(_) => {
                    match path_components.next() {
                        Some(rel)   => routes.find(|r| Kind::Relationship(rel) == r.kind),
                        None        => routes.find(|r| r.kind == Kind::Resource),
                    }
                }
                None    => routes.find(|r| r.kind == Kind::Collection),
            };
            route.map(|r| r.clone())
        })
    }
}

pub struct RoutingTable {
    pub routes: Routes,
    pub handlers: HashMap<Route, Handler>,
}

impl Service for RoutingTable {
    type Request = http::Request;
    type Response = http::Response;
    type Error = http::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        match self.routes.process(&req) {
            Some(route) => {
                match self.handlers.get(&route) {
                    Some(handler)   => handler.call(req),
                    None            => not_found(req),
                }
            }
            None        => not_found(req),
        }
    }
}

pub type Handler = Box<Service<Request = http::Request,
                               Response = http::Response,
                               Error = http::Error,
                               Future = BoxFuture<http::Response, http::Error>>>;

pub fn not_found(_: http::Request) -> BoxFuture<http::Response, http::Error> {
    future::ok(http::Response::new().with_status(http::StatusCode::NotFound)).boxed()
}
