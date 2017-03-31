use futures::{future, Future, BoxFuture};
use futures::stream::BoxStream;
use tokio::{Service, NewService};
use tokio::stream::{StreamService, NewStreamService};
use tokio as t;
use tokio::stream as s;
use mainsail::{Resource, Error};

use http;
use format::Format;
use present::middleware::Presenter;
use receive::middleware::Receiver;
use request::{ResourceRequest, CollectionRequest};

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Route {
    pub endpoint: String,
    pub method: Method,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum Method {
    Get,
    Index,
}

impl Route {
    pub fn from_request(req: &http::Request) -> Route {
        let path = {
            let path = req.path();
            if path.starts_with('/') { &path[1..] } else { path }
        };
        let mut path_components = path.rsplitn(2, '/').collect::<Vec<_>>();
        if path_components.len() == 1 {
            Route {
                endpoint: path_components.pop().unwrap().to_string(),
                method: Method::Index,
            }
        } else {
            Route {
                endpoint: path_components.pop().unwrap().to_string(),
                method: Method::Get,
            }
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

pub fn new_resource_service<F, Q, T, S>()
    -> t::NewServiceWrapper<Presenter<F::Presenter, Q>,
                            t::NewServiceWrapper<Receiver<F::Receiver>, Q::Service>>
where
    F: Format<T>,
    Q: ResourceRequest<T>,
    Q::Service: NewService<Request = Q, Response = T, Error = Error, Instance = S>,
    S: Service<Request = Q, Response = T, Error = Error, Future = BoxFuture<T, Error>>,
    T: Resource,
{
    let receiver = Receiver::new(F::Receiver::default());
    let presenter = Presenter::<_, Q>::new(F::Presenter::default());
    let service = Q::Service::default();
    service.wrap(receiver).wrap(presenter)
}

pub fn new_collection_service<F, Q, T, S>()
    -> s::NewStreamServiceReducer<Presenter<F::Presenter, Q>,
                                  s::NewStreamServiceWrapper<Receiver<F::Receiver>, Q::Service>>
where
    F: Format<T>,
    Q: CollectionRequest<T>,
    Q::Service: NewStreamService<Request = Q, Response = T, Error = Error, Instance = S>,
    S: StreamService<Request = Q, Response = T, Error = Error, Stream = BoxStream<T, Error>>,
    T: Resource,
{
    let receiver = Receiver::new(F::Receiver::default());
    let presenter = Presenter::<_, Q>::new(F::Presenter::default());
    let service = Q::Service::default();
    service.wrap(receiver).reduce(presenter)
}
