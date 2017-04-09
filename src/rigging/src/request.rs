use std::marker::PhantomData;

use mainsail::{ResourceEndpoint, Error, Environment, Get, Index};
use tokio::NewService;
use tokio::stream::NewStreamService;

use http;
use service::{GetService, IndexService};
use receive::Receive;

pub trait Request<T: ResourceEndpoint>: Sized {
    type Service: Default;
    fn receive<R: Receive<T>>(rec: &R, req: http::Request) -> Result<Self, Error>;
}

pub trait ResourceRequest<T: ResourceEndpoint>: Request<T>
where
    <Self as Request<T>>::Service: NewService<Response = T, Error = Error>,
{ }

pub trait CollectionRequest<T: ResourceEndpoint>: Request<T>
where
    <Self as Request<T>>::Service: NewStreamService<Response = T, Error = Error>,
{ }

pub struct GetRequest<T: ResourceEndpoint> {
    pub identifier: T::Identifier,
    pub env: Environment,
}

impl<T: Get + ResourceEndpoint> Request<T> for GetRequest<T> {
    type Service = GetService<T>;

    fn receive<R: Receive<T>>(rec: &R, req: http::Request) -> Result<Self, Error> {
        let id = req.path().rsplit('/').next().ok_or(Error).and_then(|s| {
            s.parse().or(Err(Error))
        })?;
        rec.get(req, id)
    }
}

impl<T: Get + ResourceEndpoint> ResourceRequest<T> for GetRequest<T> { }

pub struct IndexRequest<T: ResourceEndpoint> {
    pub env: Environment,
    pub _spoopy: PhantomData<T>,
}

impl<T: Index + ResourceEndpoint> Request<T> for IndexRequest<T> {
    type Service = IndexService<T>;

    fn receive<R: Receive<T>>(rec: &R, req: http::Request) -> Result<Self, Error> {
        rec.index(req)
        
    }
}

impl<T: Index + ResourceEndpoint> CollectionRequest<T> for IndexRequest<T> { }
