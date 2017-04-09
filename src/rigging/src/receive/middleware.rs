use std::io;

use futures::{future, BoxFuture, Future};
use futures::stream::{self, BoxStream, Stream};
use tokio::{Service, NewService, Middleware, NewMiddleware};
use tokio::stream::{StreamService, StreamMiddleware, NewStreamMiddleware, NewStreamService};

use http;
use mainsail::{ResourceEndpoint, Error};
use request::Request;
use receive::Receive;

#[derive(Clone)]
pub struct Receiver<R> {
    receiver: R,
}

impl<R> Receiver<R> {
    pub fn new(receiver: R) -> Receiver<R> {
        Receiver { receiver }
    }
}

impl<S, R, T, Q> Middleware<S> for Receiver<R>
where
    T: ResourceEndpoint,
    R: Receive<T>,
    Q: Request<T>,
    Q::Service: NewService<Request = Q, Response = T, Error = Error, Instance = S>,
    S: Service<Request = Q, Response = T, Error = Error, Future = BoxFuture<T, Error>>,
{
    type WrappedService = ReceivedReq<S, R>;
    fn wrap(self, service: S) -> Self::WrappedService {
        Self::WrappedService {
            service,
            receiver: self.receiver,
        }
    }
}

impl<S, R, T, Q> NewMiddleware<S> for Receiver<R>
where
    T: ResourceEndpoint,
    R: Receive<T>,
    Q: Request<T>,
    Q::Service: NewService<Request = Q, Response = T, Error = Error, Instance = S>,
    S: Service<Request = Q, Response = T, Error = Error, Future = BoxFuture<T, Error>>,
{
    type Instance = Self;
    type WrappedService = ReceivedReq<S, R>;

    fn new_middleware(&self) -> io::Result<Self::Instance> {
        Ok(self.clone())
    }
}

pub struct ReceivedReq<S, R> {
    receiver: R,
    service: S,
}

impl<S, R, T, Q> Service for ReceivedReq<S, R>
where
    T: ResourceEndpoint,
    R: Receive<T>,
    Q: Request<T>,
    Q::Service: NewService<Request = Q, Response = T, Error = Error, Instance = S>,
    S: Service<Request = Q, Response = T, Error = Error, Future = BoxFuture<T, Error>>,
{
    type Request = http::Request;
    type Response = S::Response;
    type Error = Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        match Q::receive(&self.receiver, req) {
            Ok(req)     => self.service.call(req),
            Err(err)    => future::err(err).boxed(),
        }
    }
}

impl<S, R, T, Q> NewStreamMiddleware<S> for Receiver<R>
where
    T: ResourceEndpoint,
    R: Receive<T>,
    Q: Request<T>,
    Q::Service: NewStreamService<Request = Q, Response = T, Error = Error, Instance = S>,
    S: StreamService<Request = Q, Response = T, Error = Error, Stream = BoxStream<T, Error>>,
{
    type WrappedService = ReceivedReqStream<S, R>;
    type Instance = Self;

    fn new_middleware(&self) -> io::Result<Self::Instance> {
        Ok(self.clone())
    }
}

impl<S, R, T, Q> StreamMiddleware<S> for Receiver<R>
where
    T: ResourceEndpoint,
    R: Receive<T>,
    Q: Request<T>,
    Q::Service: NewStreamService<Request = Q, Response = T, Error = Error, Instance = S>,
    S: StreamService<Request = Q, Response = T, Error = Error, Stream = BoxStream<T, Error>>,
{
    type WrappedService = ReceivedReqStream<S, R>;
    fn wrap(self, service: S) -> Self::WrappedService {
        Self::WrappedService {
            service,
            receiver: self.receiver,
        }
    }
}

pub struct ReceivedReqStream<S, R> {
    receiver: R,
    service: S,
}

impl<S, R, T, Q> StreamService for ReceivedReqStream<S, R>
where
    T: ResourceEndpoint,
    R: Receive<T>,
    Q: Request<T>,
    Q::Service: NewStreamService<Request = Q, Response = T, Error = Error, Instance = S>,
    S: StreamService<Request = Q, Response = T, Error = Error, Stream = BoxStream<T, Error>>,
{
    type Request = http::Request;
    type Response = T;
    type Error = Error;
    type Stream = BoxStream<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Stream {
        match Q::receive(&self.receiver, req) {
            Ok(req)     => self.service.call(req),
            Err(err)    => stream::once(Err(err)).boxed(),
        }
    }
}
