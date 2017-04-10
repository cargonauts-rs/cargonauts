use std::collections::HashMap;

use tokio::Service;
use futures::{future, BoxFuture, Future};
use http;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum MethodKind {
    Get,
    Index,
    GetRel(&'static str),
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct Routes {
    pub endpoints: &'static [Route],
}

pub type Route = (&'static str, MethodKind);

impl Routes {
    fn process(&self, req: &http::Request) -> Option<Route> {
        let mut path_components = req.path().split('/').filter(|p| !p.is_empty());
        path_components.next().and_then(|p| {
            let mut endpoint_routes = self.endpoints.iter().filter(|&&(e, _)| e == p);
            match path_components.next() {
                Some(_) => {
                    match path_components.next() {
                        Some(r) => {
                            endpoint_routes.find(|&&(_, m)| match m {
                                MethodKind::GetRel(e_r) => e_r == r,
                                _                   => false,
                            }).map(|x|*x)
                        }
                        None    => {
                            endpoint_routes.find(|&&(_, m)| m == MethodKind::Get).map(|x|*x)
                        }
                    }
                }
                None    => {
                    endpoint_routes.find(|&&(_, m)| m == MethodKind::Index).map(|x|*x)
                }
            }
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
