use std::io;
use std::marker::PhantomData;

use futures::{self, BoxFuture, Stream, Future};
use tokio::{Service, NewService, Middleware, NewMiddleware};
use tokio::stream::{StreamService, StreamReduce, NewStreamReduce, NewStreamService};

use http;
use method::Method;
use mainsail::{ResourceEndpoint, Error};
use present::{Present, PresentResource, PresentCollection};

pub struct Presenter<P, M: ?Sized> {
    presenter: P,
    _spoopy: PhantomData<M>,
}

impl<P, M: ?Sized> Presenter<P, M> {
    pub fn new(presenter: P) -> Self {
        Presenter { presenter, _spoopy: PhantomData }
    }
}

impl<S, P, M> NewMiddleware<S> for Presenter<P, M>
where
    M: ?Sized + Method,
    M::Service: NewService<Response = M::Response, Error = Error>,
    P: Present<M::Response>,
    S: Service<Response = M::Response, Error = Error>,
    S::Future: Send + 'static,
{
    type Instance = ResourcePresenter<P::ResourcePresenter, M>;
    type WrappedService = PresentedResource<S, P::ResourcePresenter, M>;

    fn new_middleware(&self) -> io::Result<Self::Instance> {
        Ok(ResourcePresenter {
            presenter: self.presenter.for_resource(),
            _spoopy: PhantomData,
        })
    }
}

pub struct ResourcePresenter<P, M: ?Sized> {
    presenter: P,
    _spoopy: PhantomData<M>,
}

impl<S, P, M> Middleware<S> for ResourcePresenter<P, M>
where
    M: ?Sized + Method,
    M::Service: NewService<Response = M::Response, Error = Error>,
    P: PresentResource<M::Response>,
    S: Service<Response = M::Response, Error = Error>,
    S::Future: Send + 'static,
{
    type WrappedService = PresentedResource<S, P, M>;

    fn wrap(self, service: S) -> Self::WrappedService {
        PresentedResource {
            service,
            presenter: self.presenter,
            _spoopy: PhantomData,
        }
    }
}

pub struct PresentedResource<S, P, M: ?Sized> {
    presenter: P,
    service: S,
    _spoopy: PhantomData<M>,
}

impl<S, P, M> Service for PresentedResource<S, P, M>
where
    M: ?Sized + Method,
    M::Service: NewService<Response = M::Response, Error = Error>,
    P: PresentResource<M::Response>,
    S: Service<Response = M::Response, Error = Error>,
    S::Future: Send + 'static,
{
    type Request = S::Request;
    type Response = http::Response;
    type Error = http::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let presenter = self.presenter.clone();
        self.service.call(req).then(move |result| match result {
            Ok(resource) => Ok(presenter.resource::<M>(resource, None)),
            Err(error) => Ok(presenter.error::<M>(error, None)),
        }).boxed()
    }
}

impl<S, P, M> NewStreamReduce<S> for Presenter<P, M>
where
    P: Present<M::Response>,
    M: ?Sized + Method,
    M::Service: NewStreamService<Response = M::Response, Error = Error>,
    S: StreamService<Response = M::Response, Error = Error>,
    S::Stream: Send + 'static,
{
    type Instance = CollectionPresenter<P::CollectionPresenter, M>;
    type ReducedService = PresentedCollection<S, P::CollectionPresenter, M>;

    fn new_reducer(&self) -> io::Result<Self::Instance> {
        Ok(CollectionPresenter  {
            presenter: self.presenter.for_collection(),
            _spoopy: PhantomData,
        })
    }
}

pub struct CollectionPresenter<P, M: ?Sized> {
    presenter: P,
    _spoopy: PhantomData<M>,
}

impl<S, P, M> StreamReduce<S> for CollectionPresenter<P, M>
where
    P: PresentCollection<M::Response>,
    M: ?Sized + Method,
    M::Service: NewStreamService<Response = M::Response, Error = Error>,
    S: StreamService<Response = M::Response, Error = Error>,
    S::Stream: Send + 'static,
{
    type ReducedService = PresentedCollection<S, P, M>;

    fn reduce(self, service: S) -> Self::ReducedService {
        PresentedCollection  {
            service,
            presenter: self.presenter,
            _spoopy: PhantomData,
        }
    }
}

pub struct PresentedCollection<S, P, M: ?Sized> {
    presenter: P,
    service: S,
    _spoopy: PhantomData<M>,
}

impl<S, P, M> Service for PresentedCollection<S, P, M>
where
    P: PresentCollection<M::Response>,
    M: ?Sized + Method,
    M::Service: NewStreamService<Response = M::Response, Error = Error>,
    S: StreamService<Response = M::Response, Error = Error>,
    S::Stream: Send + 'static,
{
    type Request = S::Request;
    type Response = http::Response;
    type Error = http::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let mut presenter = self.presenter.clone();
        presenter.start::<M>();
        self.service.call(req).then(|result| -> Result<_, http::Error> {
            Ok(result)
        }).forward(CollectionSink {
            presenter,
            _spoopy: PhantomData,
        }).then(|result| match result {
            Ok((_, s))  => Ok(s.presenter.finish()),
            Err(_)      => unreachable!(),
        }).boxed()
    }
}

struct CollectionSink<P: PresentCollection<T>, T: ResourceEndpoint> {
    presenter: P,
    _spoopy: PhantomData<T>
}

impl<P: PresentCollection<T>, T: ResourceEndpoint> futures::Sink for CollectionSink<P, T> {
    type SinkItem = Result<T, Error>;
    type SinkError = http::Error;

    fn start_send(&mut self, item: Self::SinkItem) -> futures::StartSend<Self::SinkItem, Self::SinkError> {
        match item {
            Ok(resource)    => self.presenter.append(resource, None),
            Err(error)      => self.presenter.error(error, None),
        }
        Ok(futures::AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> futures::Poll<(), Self::SinkError> {
        Ok(futures::Async::Ready(()))
    }
}

