use std::collections::HashMap;
use std::hash::Hash;
use std::io;

use futures::{future, IntoFuture};
use tokio::{NewService, Service};
use c3po::Conn;

pub struct MockConnection<C>
where
    C: NewService,
    C::Request: Clone + Hash + Eq,
    C::Response: Clone,
    C::Error: Clone,
{
    map: HashMap<C::Request, Result<C::Response, C::Error>>
}

impl<C> MockConnection<C>
where
    C: NewService,
    C::Request: Clone + Hash + Eq,
    C::Response: Clone,
    C::Error: Clone,
{
    pub fn set(&mut self, req: C::Request, res: Result<C::Response, C::Error>) {
        self.map.insert(req, res);
    }

    pub fn conn(self) -> Conn<Self> {
        Conn::new_unpooled(self)
    }
}

impl<C> Default for MockConnection<C>
where
    C: NewService,
    C::Request: Clone + Hash + Eq,
    C::Response: Clone,
    C::Error: Clone,
{
    fn default() -> Self {
        Self { map: HashMap::default() }
    }
}

impl<C> Clone for MockConnection<C>
where
    C: NewService,
    C::Request: Clone + Hash + Eq,
    C::Response: Clone,
    C::Error: Clone,
{
    fn clone(&self) -> Self {
        Self { map: self.map.clone() }
    }
}

impl<C> NewService for MockConnection<C>
where
    C: NewService,
    C::Request: Clone + Hash + Eq,
    C::Response: Clone,
    C::Error: Clone,
{
    type Request = C::Request;
    type Response = C::Response;
    type Error = C::Error;
    type Instance = Self;
    type Future = future::FutureResult<Self, io::Error>;

    fn new_service(&self) -> Self::Future {
        future::ok(self.clone())
    }
}

impl<C> Service for MockConnection<C>
where
    C: NewService,
    C::Request: Clone + Hash + Eq,
    C::Response: Clone,
    C::Error: Clone,
{
    type Request = C::Request;
    type Response = C::Response;
    type Error = C::Error;
    type Future = future::FutureResult<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        self.map.get(&req).expect("no response for this request").clone().into_future()
    }
}
