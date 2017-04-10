use std::marker::PhantomData;

use mainsail::{ResourceEndpoint, Error, Environment};

use http;
use receive::Receive;

pub trait Request: Sized {
    type Resource: ResourceEndpoint;
    fn receive<R: Receive<Self::Resource>>(rec: &R, req: http::Request) -> Result<Self, Error>;
}

pub struct GetRequest<T: ResourceEndpoint> {
    pub identifier: T::Identifier,
    pub env: Environment,
}

impl<T: ResourceEndpoint> Request for GetRequest<T> {
    type Resource = T;
    fn receive<R: Receive<T>>(rec: &R, req: http::Request) -> Result<Self, Error> {
        let id = req.path().split('/').nth(2).ok_or(Error).and_then(|s| {
            s.parse().or(Err(Error))
        })?;
        rec.get(req, id)
    }
}

pub struct IndexRequest<T: ResourceEndpoint> {
    pub env: Environment,
    pub _spoopy: PhantomData<T>,
}

impl<T: ResourceEndpoint> Request for IndexRequest<T> {
    type Resource = T;
    fn receive<R: Receive<T>>(rec: &R, req: http::Request) -> Result<Self, Error> {
        rec.index(req)
        
    }
}
