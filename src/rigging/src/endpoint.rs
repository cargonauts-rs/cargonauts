use std::io;
use std::marker::PhantomData;

use futures::{Future, BoxFuture, Stream, IntoFuture, future};
use tokio::{Service, NewService};

use Error;
use ResourceEndpoint;
use Method;

use request::*;
use format::*;
use environment::Environment;
use http;

pub trait Endpoint<Hits, Returns, T: ResourceEndpoint> {
    type Future: Future<Item = http::Response, Error = http::Error>;
    fn call(http::Request) -> Self::Future;
}

fn parse_id<T: ResourceEndpoint>(req: &http::Request) -> Result<T::Identifier, Error> {
    req.path().split('/').filter(|p| !p.is_empty()).skip(1).next().ok_or(Error).and_then(|s| {
        s.parse().map_err(|_| Error)
    })
}

pub struct _Resource;

pub struct _Collection;

impl<M, T, F> Endpoint<_Resource, _Resource, T> for (F, M)
where
    T: ResourceEndpoint,
    M: ?Sized + Method<T>,
    M::Request: ResourceRequest<T>,
    M::Outcome: Future<Item = M::Response, Error = Error>,
    F: Format<T, M>,
{
    type Future = BoxFuture<http::Response, http::Error>;
    fn call(req: http::Request) -> Self::Future {
        parse_id::<T>(&req).into_future().and_then(|id| {
            let mut env = Environment::default();
            F::Receiver::receive(req, &mut env).into_future().and_then(|parts| {
                let request = M::Request::new(parts, id, &mut env);
                M::call(request, env).then(|result| {
                    let presenter = F::Presenter::for_resource();
                    match result {
                        Ok(resource)    => Ok(presenter.resource(resource, None)),
                        Err(error)      => Ok(presenter.error(error, None)),
                    }
                })
            })
        }).or_else(|err| {
            Ok(F::Presenter::for_resource().error(err, None))
        }).boxed()
    }
}

impl<M, T, F> Endpoint<_Collection, _Collection, T> for (F, M)
where
    T: ResourceEndpoint,
    M: ?Sized + Method<T>,
    M::Request: CollectionRequest<T>,
    M::Outcome: Stream<Item = M::Response, Error = Error>,
    F: Format<T, M>,
{
    type Future = BoxFuture<http::Response, http::Error>;
    fn call(req: http::Request) -> Self::Future {
        let mut env = Environment::default();
        F::Receiver::receive(req, &mut env).into_future().and_then(|parts| {
            let request = M::Request::new(parts, &mut env);
            let stream = M::call(request, env);
            let presenter = F::Presenter::for_collection();
            stream.then(|result| -> Result<_, ()> { Ok(result) }).fold(presenter, |mut presenter, result| {
                match result {
                    Ok(resource)    => presenter.append(resource, None),
                    Err(error)      => presenter.error(error, None),
                }
                Ok(presenter)
            }).then(|presenter| Ok(presenter.unwrap().finish()))
        }).or_else(|err| {
            Ok(F::Presenter::for_resource().error(err, None))
        }).boxed()
    }
}

impl<M, T, F> Endpoint<_Collection, _Resource, T> for (F, M)
where
    T: ResourceEndpoint,
    M: ?Sized + Method<T>,
    M::Request: CollectionRequest<T>,
    M::Outcome: Future<Item = M::Response, Error = Error>,
    F: Format<T, M>,
{
    type Future = BoxFuture<http::Response, http::Error>;
    fn call(req: http::Request) -> Self::Future {
        let mut env = Environment::default();
        F::Receiver::receive(req, &mut env).into_future().and_then(|parts| {
            let request = M::Request::new(parts, &mut env);
            M::call(request, env).then(|result| {
                let presenter = F::Presenter::for_resource();
                match result {
                    Ok(resource)    => Ok(presenter.resource(resource, None)),
                    Err(error)      => Ok(presenter.error(error, None)),
                }
            })
        }).or_else(|err| {
            Ok(F::Presenter::for_resource().error(err, None))
        }).boxed()
    }
}

impl<M, T, F> Endpoint<_Resource, _Collection, T> for (F, M)
where
    T: ResourceEndpoint,
    M: ?Sized + Method<T>,
    M::Request: ResourceRequest<T>,
    M::Outcome: Stream<Item = M::Response, Error = Error>,
    F: Format<T, M>,
{
    type Future = BoxFuture<http::Response, http::Error>;
    fn call(req: http::Request) -> Self::Future {
        parse_id::<T>(&req).into_future().and_then(|id| {
            let mut env = Environment::default();
            F::Receiver::receive(req, &mut env).into_future().and_then(|parts| {
                let request = M::Request::new(parts, id, &mut env);
                let stream = M::call(request, env);
                let presenter = F::Presenter::for_collection();
                stream.then(|result| -> Result<_, ()> { Ok(result) }).fold(presenter, |mut presenter, result| {
                    match result {
                        Ok(resource)    => presenter.append(resource, None),
                        Err(error)      => presenter.error(error, None),
                    }
                    Ok(presenter)
                }).then(|presenter| Ok(presenter.unwrap().finish()))
            })
        }).or_else(|err| {
            Ok(F::Presenter::for_resource().error(err, None))
        }).boxed()
    }
}

pub struct EndpointService<Hits, Returns, T, E: ?Sized> {
    _marker: PhantomData<(Hits, Returns, T, E)>,
}

impl<H, R, T, E> Default for EndpointService<H, R, T, E>
where
    T: ResourceEndpoint,
    E: ?Sized + Endpoint<H, R, T>,
{
    fn default() -> Self {
        EndpointService { _marker: PhantomData }
    }
}

impl<H, R, T, E> Clone for EndpointService<H, R, T, E>
where
    T: ResourceEndpoint,
    E: ?Sized + Endpoint<H, R, T>,
{
    fn clone(&self) -> Self {
        EndpointService { _marker: PhantomData }
    }
}

impl<H, R, T, E> Service for EndpointService<H, R, T, E>
where
    T: ResourceEndpoint,
    E: ?Sized + Endpoint<H, R, T>,
{
    type Request = http::Request;
    type Response = http::Response;
    type Error = http::Error;
    type Future = E::Future;

    fn call(&self, req: Self::Request) -> Self::Future {
        E::call(req)
    }
}


impl<H, R, T, E> NewService for EndpointService<H, R, T, E>
where
    T: ResourceEndpoint,
    E: ?Sized + Endpoint<H, R, T>,
{
    type Request = http::Request;
    type Response = http::Response;
    type Error = http::Error;
    type Instance = Self;
    type Future = future::FutureResult<Self::Instance, io::Error>;

    fn new_service(&self) -> Self::Future {
        future::ok(self.clone())
    }
}
