use std::io;
use std::marker::PhantomData;

use futures::{future, BoxFuture, Future};
use futures::stream::{self, BoxStream, Stream};
use tokio::{Service, NewService, Middleware, NewMiddleware};
use tokio::stream::{StreamService, StreamMiddleware, NewStreamMiddleware, NewStreamService};

use mainsail::{ResourceEndpoint, Error};

use http;
use method::Method;
use request::Request;
use receive::Receive;

pub struct Receiver<R, M: ?Sized> {
    receiver: R,
    _spoopy: PhantomData<M>,
}

impl<R: Clone, M: ?Sized> Clone for Receiver<R, M> {
    fn clone(&self) -> Receiver<R, M> {
        Receiver {
            receiver: self.receiver.clone(),
            _spoopy: PhantomData,
        }
    }
}

impl<R, M: ?Sized> Receiver<R, M> {
    pub fn new(receiver: R) -> Receiver<R, M> {
        Receiver { receiver, _spoopy: PhantomData, }
    }
}

impl<S, R, T, Q, M> Middleware<S> for Receiver<R, M>
where
    T: ResourceEndpoint,
    R: Receive<T>,
    M: ?Sized + Method<Request = Q>,
    Q: Request<Resource = T>,
    M::Service: NewService<Request = Q, Response = M::Response, Error = Error, Instance = S>,
    S: Service<Request = Q, Response = M::Response, Error = Error, Future = BoxFuture<M::Response, Error>>,
{
    type WrappedService = ReceivedReq<S, R, M>;
    fn wrap(self, service: S) -> Self::WrappedService {
        Self::WrappedService {
            service,
            receiver: self.receiver,
            _spoopy: PhantomData,
        }
    }
}

impl<S, R, T, Q, M> NewMiddleware<S> for Receiver<R, M>
where
    T: ResourceEndpoint,
    R: Receive<T>,
    M: ?Sized + Method<Request = Q>,
    Q: Request<Resource = T>,
    M::Service: NewService<Request = Q, Response = M::Response, Error = Error, Instance = S>,
    S: Service<Request = Q, Response = M::Response, Error = Error, Future = BoxFuture<M::Response, Error>>,
{
    type Instance = Self;
    type WrappedService = ReceivedReq<S, R, M>;

    fn new_middleware(&self) -> io::Result<Self::Instance> {
        Ok(self.clone())
    }
}

pub struct ReceivedReq<S, R, M: ?Sized> {
    receiver: R,
    service: S,
    _spoopy: PhantomData<M>,
}

impl<S, R, T, M, Q> Service for ReceivedReq<S, R, M>
where
    T: ResourceEndpoint,
    R: Receive<T>,
    M: ?Sized + Method<Request = Q>,
    Q: Request<Resource = T>,
    M::Service: NewService<Request = Q, Response = M::Response, Error = Error, Instance = S>,
    S: Service<Request = Q, Response = M::Response, Error = Error, Future = BoxFuture<M::Response, Error>>,
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

impl<S, R, T, Q, M> NewStreamMiddleware<S> for Receiver<R, M>
where
    T: ResourceEndpoint,
    R: Receive<T>,
    M: ?Sized + Method<Request = Q>,
    Q: Request<Resource = T>,
    M::Service: NewStreamService<Request = Q, Response = M::Response, Error = Error, Instance = S>,
    S: StreamService<Request = Q, Response = M::Response, Error = Error, Stream = BoxStream<M::Response, Error>>,
{
    type WrappedService = ReceivedReqStream<S, R, M>;
    type Instance = Self;

    fn new_middleware(&self) -> io::Result<Self::Instance> {
        Ok(self.clone())
    }
}

impl<S, R, T, Q, M> StreamMiddleware<S> for Receiver<R, M>
where
    T: ResourceEndpoint,
    M: ?Sized + Method<Request = Q>,
    R: Receive<T>,
    Q: Request<Resource = T>,
    M::Service: NewStreamService<Request = Q, Response = M::Response, Error = Error, Instance = S>,
    S: StreamService<Request = Q, Response = M::Response, Error = Error, Stream = BoxStream<M::Response, Error>>,
{
    type WrappedService = ReceivedReqStream<S, R, M>;
    fn wrap(self, service: S) -> Self::WrappedService {
        Self::WrappedService {
            service,
            receiver: self.receiver,
            _spoopy: PhantomData,
        }
    }
}

pub struct ReceivedReqStream<S, R, M: ?Sized> {
    receiver: R,
    service: S,
    _spoopy: PhantomData<M>,
}

impl<S, R, T, Q, M> StreamService for ReceivedReqStream<S, R, M>
where
    T: ResourceEndpoint,
    R: Receive<T>,
    M: ?Sized + Method<Request = Q>,
    Q: Request<Resource = T>,
    M::Service: NewStreamService<Request = Q, Response = M::Response, Error = Error, Instance = S>,
    S: StreamService<Request = Q, Response = M::Response, Error = Error, Stream = BoxStream<M::Response, Error>>,
{
    type Request = http::Request;
    type Response = M::Response;
    type Error = Error;
    type Stream = BoxStream<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Stream {
        match Q::receive(&self.receiver, req) {
            Ok(req)     => self.service.call(req),
            Err(err)    => stream::once(Err(err)).boxed(),
        }
    }
}
