use std::collections::HashMap;

use Future;
use router::{Router, ResourceRoute, Method, Request, Response, MakeLinks};

pub trait Server: 'static {
    type Request: Request + 'static;
    type Response: Response + 'static;
    type LinkMaker: MakeLinks;
    
    fn route<R: Router>(&mut self, router: R);
}

enum Endpoint<S: Server> {
    Resource(ResourceRoutes<S>),
    Alias(Method, fn(S::Request, S::LinkMaker) -> Box<Future<Item = S::Response, Error = ()>>),
}

impl<S: Server> Endpoint<S> {
    fn new_resource() -> Endpoint<S> {
        Endpoint::Resource(ResourceRoutes {
            routes: HashMap::new(),
        })
    }
}

struct ResourceRoutes<S: Server> {
    routes: HashMap<ResourceRoute<'static>, fn(S::Request, S::LinkMaker) -> Box<Future<Item = S::Response, Error = ()>>>,
}

pub struct DefaultRouter<S: Server> {
    endpoints: HashMap<&'static str, Endpoint<S>>,
}

impl<S: Server> DefaultRouter<S> {
    pub fn lookup(&self, request: &S::Request) -> Option<fn(S::Request, S::LinkMaker) -> Box<Future<Item = S::Response, Error = ()>>> {
        self.endpoints.get(request.endpoint()).and_then(|endpoint| {
            match *endpoint {
                Endpoint::Resource(ref resource_routes)     => {
                    let route = ResourceRoute {
                        name: request.endpoint(),
                        method: request.method(),
                        component: request.component(),
                    };
                    resource_routes.routes.get(&route).map(|handler| *handler)
                }
                Endpoint::Alias(ref method, ref handler)    => {
                    if *method == request.method() {
                        Some(*handler)
                    } else { None }
                }
            }
        })
    }
}

impl<S: Server> Router for DefaultRouter<S> {
    type Request = S::Request;
    type Response = S::Response;
    type LinkMaker = S::LinkMaker;

    fn attach_resource(&mut self,
        route: ResourceRoute<'static>,
        handler: fn(Self::Request, Self::LinkMaker) -> Box<Future<Item = Self::Response, Error = ()>>
    ) {
        match *self.endpoints.entry(route.name).or_insert_with(Endpoint::new_resource) {
            Endpoint::Resource(ref mut resource_routes) => {
                if resource_routes.routes.insert(route, handler).is_some() {
                    panic!("resource route attached twice: {:?}", route);
                }
            }
            _                                           => unreachable!(),
        }
    }

    fn attach_alias(&mut self,
        alias: &'static str,
        method: Method,
        handler: fn(Self::Request, Self::LinkMaker) -> Box<Future<Item = Self::Response, Error = ()>>
    ) {
        if self.endpoints.insert(alias, Endpoint::Alias(method, handler)).is_some() {
            panic!("alias attached twice: {}, {:?}", alias, method);
        }
    }
}
