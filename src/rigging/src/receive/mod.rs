use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use mainsail::{AnyMap, Resource, Error};

use http;
use request::{GetRequest, IndexRequest};

pub mod middleware;

pub trait Receive<T: Resource>: Default + Clone {
    fn get(&self, _: http::Request, id: T::Identifier) -> Result<GetRequest<T>, Error> {
        Ok(GetRequest {
            identifier: id,
            env: Arc::new(Mutex::new(AnyMap::new())),
        })
    }

    fn index(&self, _: http::Request) -> Result<IndexRequest<T>, Error> {
        Ok(IndexRequest {
            env: Arc::new(Mutex::new(AnyMap::new())),
            _spoopy: PhantomData,
        })
    }
}
