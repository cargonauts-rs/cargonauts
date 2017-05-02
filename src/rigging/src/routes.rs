use std::borrow::Borrow;
use std::collections::HashMap;
use std::io;
use std::rc::Rc;

use tokio::{Service, NewService};
use futures::{Future, future};

use http;
use environment::{PreparedEnv, Environment};

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
    assets: Rc<HashMap<&'static str, &'static [u8]>>,
    asset_handler: AssetHandler,
    env: PreparedEnv,
}

impl RoutingTable {
    pub fn new(
        routes: HashMap<RouteKey, Handler>,
        assets: HashMap<&'static str, &'static [u8]>,
        asset_handler: AssetHandler,
        env: PreparedEnv
    ) -> RoutingTable {
        RoutingTable {
            routes: Rc::new(routes),
            assets: Rc::new(assets),
            asset_handler,
            env,
        }
    }
}

impl Service for RoutingTable {
    type Request = http::Request;
    type Response = http::Response;
    type Error = http::Error;
    type Future = http::BoxFuture;

    fn call(&self, req: Self::Request) -> Self::Future {
        {
            let path = req.path().trim_matches('/');
            if let Some(asset) = self.assets.get(path) {
                return (self.asset_handler)(asset)
            }
        }
        match RouteKeyRef::req(&req).and_then(|route| self.routes.get(&route)) {
            Some(handle)    => handle(req, self.env.new()),
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

pub type Handler = fn(http::Request, Environment) -> http::BoxFuture;
pub type AssetHandler = fn(&'static [u8]) -> http::BoxFuture;

pub fn not_found() -> http::BoxFuture {
    future::ok(http::Response::new().with_status(http::StatusCode::NotFound)).boxed()
}

pub fn default_asset_handler(asset: &'static [u8]) -> http::BoxFuture {
    Box::new(future::ok(http::Response::new()
                            .with_status(http::StatusCode::Ok)
                            .with_header(http::headers::ContentLength(asset.len() as u64))
                            .with_body(asset)))
}
