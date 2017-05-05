use std::borrow::Borrow;
use std::collections::HashMap;
use std::io;
use std::rc::Rc;

use tokio::{Service, NewService};
use futures::{Future, future};

use http;
use endpoint;
use environment::{PreparedEnv};

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct RouteKey {
    key: RouteKeyRef<'static>,
}
#[derive(Clone, Hash, Eq, PartialEq)]
struct RouteKeyRef<'a> {
    endpoint: &'a str,
    route: Route<'a>,
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

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct AssetKey {
    key: AssetKeyRef<'static>,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct AssetKeyRef<'a> {
    endpoint: &'a str,
}

impl AssetKey {
    pub fn new(endpoint: &'static str) -> AssetKey {
        AssetKey { key: AssetKeyRef { endpoint } }
    }
}

impl<'a> Borrow<AssetKeyRef<'a>> for AssetKey {
    fn borrow(&self) -> &AssetKeyRef<'a> {
        &self.key
    }
}

impl<'a> AssetKeyRef<'a> {
    fn new(req: &'a http::Request) -> AssetKeyRef<'a> {
        let endpoint = req.path().trim_matches('/');
        AssetKeyRef { endpoint }
    }
}

#[derive(Clone)]
pub struct RoutingTable {
    routes: Rc<HashMap<RouteKey, Handler>>,
    assets: Rc<HashMap<AssetKey, &'static [u8]>>,
    asset_handler: AssetHandler,
    env: PreparedEnv,
}

impl RoutingTable {
    pub fn new(
        routes: HashMap<RouteKey, Handler>,
        assets: HashMap<AssetKey, &'static [u8]>,
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
        let req = endpoint::Request { req, env: self.env.new() };
        if let Some(asset) = self.assets.get(&AssetKeyRef::new(&req.req)) {
            return (self.asset_handler)(asset, req)
        }
        match RouteKeyRef::req(&req.req).and_then(|route| self.routes.get(&route)) {
            Some(handle)    => handle.call(req),
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

pub type Handler = Box<Service<Request = endpoint::Request, Response = http::Response, Error = http::Error, Future = http::BoxFuture>>;
pub type AssetHandler = fn(&'static [u8], request: endpoint::Request) -> http::BoxFuture;

pub fn not_found() -> http::BoxFuture {
    future::ok(http::Response::new().with_status(http::StatusCode::NotFound)).boxed()
}

pub fn default_asset_handler(asset: &'static [u8], _: endpoint::Request) -> http::BoxFuture {
    Box::new(future::ok(http::Response::new()
                            .with_status(http::StatusCode::Ok)
                            .with_header(http::headers::ContentLength(asset.len() as u64))
                            .with_body(asset)))
}
