use std::collections::HashMap;
use std::io;
use std::rc::Rc;

use futures::{future, Future};
use recognizer::{Match, Router};
use tokio::{Service, NewService};

use endpoint;
use environment::PreparedEnv;
use http;

pub struct Route {
    pub method: http::Method,
    pub kind: Kind,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Kind {
    Resource,
    Collection,
    Relationship,
}

#[derive(Clone)]
pub struct RoutingTable {
    table: Rc<HashMap<http::Method, Router<Handler>>>,
    env: PreparedEnv,
}

impl Service for RoutingTable {
    type Request = http::Request;
    type Response = http::Response;
    type Error = http::Error;
    type Future = http::BoxFuture;

    fn call(&self, req: Self::Request) -> Self::Future {
        match self.recognize(&req) {
            Some(handle)    => {
                let id = handle.params.find("id").map(|s| s.to_owned());
                let req = endpoint::Request { req, env: self.env.new(), id };
                handle.handler.call(req)
            }
            None            => not_found()
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

impl RoutingTable {
    fn recognize(&self, req: &http::Request) -> Option<Match<&Handler>> {
        self.table.get(req.method()).and_then(|router| {
            router.recognize(req.path()).ok()
        })
    }
}

#[derive(Default)]
pub struct RouteBuilder {
    table: HashMap<http::Method, Router<Handler>>,
}

impl RouteBuilder {
    pub fn add(&mut self, method: http::Method, path: String, handler: Handler) {
        self.table.entry(method).or_insert(Router::new()).add(&path, handler);
    }

    pub fn build(self, env: PreparedEnv) -> RoutingTable {
        RoutingTable { table: Rc::new(self.table), env }
    }
}

pub type Handler = Box<Service<Request = endpoint::Request, Response = http::Response, Error = http::Error, Future = http::BoxFuture>>;

pub fn not_found() -> http::BoxFuture {
    future::ok(http::Response::new().with_status(http::StatusCode::NotFound)).boxed()
}

pub fn resource_path(endpoint: &'static str) -> String {
    format!("{}/$id", endpoint)
}

pub fn path(kind: Kind, endpoint: &'static str, rel: Option<&'static str>) -> String {
    match kind {
        Kind::Collection    => endpoint.to_owned(),
        Kind::Resource      => format!("{}/:id", endpoint),
        Kind::Relationship  => format!("{}/:id/{}", endpoint, rel.unwrap())
    }
}

pub fn default_asset_handler(asset: &'static [u8], _: endpoint::Request) -> http::BoxFuture {
    Box::new(future::ok(http::Response::new()
                            .with_status(http::StatusCode::Ok)
                            .with_header(http::headers::ContentLength(asset.len() as u64))
                            .with_body(asset)))
}

pub struct AssetHandler<F> {
    pub handler: F,
    pub data: &'static [u8]
}

impl<F> Service for AssetHandler<F>
where
    F: Fn(&'static [u8], endpoint::Request) -> http::BoxFuture
{
    type Request = endpoint::Request;
    type Response = http::Response;
    type Error = http::Error;
    type Future = http::BoxFuture;

    fn call(&self, req: Self::Request) -> Self::Future {
        (self.handler)(self.data, req)
    }
}
