use std::io;
use std::marker::PhantomData;

use futures::{self, BoxFuture, Stream, Future};
use tokio::{Service, NewService, Middleware, NewMiddleware};
use tokio::stream::{StreamService, StreamReduce, NewStreamReduce, NewStreamService};

use http;
use mainsail::{ResourceEndpoint, Error};
use present::{Present, PresentResource, PresentCollection};
use request::{ResourceRequest, CollectionRequest};

pub struct Presenter<P, Q> {
    presenter: P,
    _spoopy: PhantomData<Q>,
}

impl<P, Q> Presenter<P, Q> {
    pub fn new(presenter: P) -> Self {
        Presenter { presenter, _spoopy: PhantomData }
    }
}

impl<S, P, T, Q> NewMiddleware<S> for Presenter<P, Q>
where
    T: ResourceEndpoint,
    P: Present<T>,
    Q: ResourceRequest<T>,
    Q::Service: NewService<Response = T, Error = Error>,
    S: Service<Response = T, Error = Error>,
    S::Future: Send + 'static,
{
    type Instance = ResourcePresenter<P::ResourcePresenter, Q>;
    type WrappedService = PresentedResource<S, P::ResourcePresenter, Q>;

    fn new_middleware(&self) -> io::Result<Self::Instance> {
        Ok(ResourcePresenter {
            presenter: self.presenter.for_resource(),
            _spoopy: PhantomData,
        })
    }
}

pub struct ResourcePresenter<P, Q> {
    presenter: P,
    _spoopy: PhantomData<Q>,
}

impl<S, P, T, Q> Middleware<S> for ResourcePresenter<P, Q>
where
    T: ResourceEndpoint,
    P: PresentResource<T>,
    Q: ResourceRequest<T>,
    Q::Service: NewService<Response = T, Error = Error>,
    S: Service<Response = T, Error = Error>,
    S::Future: Send + 'static,
{
    type WrappedService = PresentedResource<S, P, Q>;

    fn wrap(self, service: S) -> Self::WrappedService {
        PresentedResource {
            service,
            presenter: self.presenter,
            _spoopy: PhantomData,
        }
    }
}

pub struct PresentedResource<S, P, Q> {
    presenter: P,
    service: S,
    _spoopy: PhantomData<Q>,
}

impl<S, P, T, Q> Service for PresentedResource<S, P, Q>
where
    T: ResourceEndpoint,
    P: PresentResource<T>,
    Q: ResourceRequest<T>,
    Q::Service: NewService<Response = T, Error = Error>,
    S: Service<Response = T, Error = Error>,
    S::Future: Send + 'static,
{
    type Request = S::Request;
    type Response = http::Response;
    type Error = http::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let presenter = self.presenter.clone();
        self.service.call(req).then(move |result| match result {
            Ok(resource) => Ok(presenter.resource::<Q>(resource, None)),
            Err(error) => Ok(presenter.error::<Q>(error, None)),
        }).boxed()
    }
}

impl<S, P, T, Q> NewStreamReduce<S> for Presenter<P, Q>
where
    T: ResourceEndpoint,
    P: Present<T>,
    Q: CollectionRequest<T>,
    Q::Service: NewStreamService<Response = T, Error = Error>,
    S: StreamService<Response = T, Error = Error>,
    S::Stream: Send + 'static,
{
    type Instance = CollectionPresenter<P::CollectionPresenter, Q>;
    type ReducedService = PresentedCollection<S, P::CollectionPresenter, Q>;

    fn new_reducer(&self) -> io::Result<Self::Instance> {
        Ok(CollectionPresenter  {
            presenter: self.presenter.for_collection(),
            _spoopy: PhantomData,
        })
    }
}

pub struct CollectionPresenter<P, Q> {
    presenter: P,
    _spoopy: PhantomData<Q>,
}

impl<S, P, T, Q> StreamReduce<S> for CollectionPresenter<P, Q>
where
    T: ResourceEndpoint,
    P: PresentCollection<T>,
    Q: CollectionRequest<T>,
    Q::Service: NewStreamService<Response = T, Error = Error>,
    S: StreamService<Response = T, Error = Error>,
    S::Stream: Send + 'static,
{
    type ReducedService = PresentedCollection<S, P, Q>;

    fn reduce(self, service: S) -> Self::ReducedService {
        PresentedCollection  {
            service,
            presenter: self.presenter,
            _spoopy: PhantomData,
        }
    }
}

pub struct PresentedCollection<S, P, Q> {
    presenter: P,
    service: S,
    _spoopy: PhantomData<Q>,
}

impl<S, P, T, Q> Service for PresentedCollection<S, P, Q>
where
    T: ResourceEndpoint,
    P: PresentCollection<T>,
    Q: CollectionRequest<T>,
    Q::Service: NewStreamService<Response = T, Error = Error>,
    S: StreamService<Response = T, Error = Error>,
    S::Stream: Send + 'static,
{
    type Request = S::Request;
    type Response = http::Response;
    type Error = http::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let mut presenter = self.presenter.clone();
        presenter.start::<Q>();
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
