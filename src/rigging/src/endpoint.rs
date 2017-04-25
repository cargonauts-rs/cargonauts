use futures::{Future, Stream, IntoFuture};

use Error;
use ResourceEndpoint;
use method::Method;

use request::*;
use format::*;
use environment::Environment;
use http;

pub trait Endpoint<Hits, Returns> {
    type Future: Future<Item = http::Response, Error = http::Error>;
    fn call(http::Request, template: Option<Template>, env: Environment) -> Self::Future;
}

fn parse_id<T: ResourceEndpoint>(req: &http::Request) -> Result<T::Identifier, Error> {
    req.path().split('/').filter(|p| !p.is_empty()).skip(1).next().ok_or(Error).and_then(|s| {
        s.parse().map_err(|_| Error)
    })
}

pub struct _Resource;

pub struct _Collection;

impl<M, T, F> Endpoint<_Resource, _Resource> for (T, F, M)
where
    T: ResourceEndpoint,
    M: ?Sized + Method<T>,
    M::Request: ResourceRequest<T>,
    M::Outcome: Future<Item = M::Response, Error = Error>,
    F: Format<T, M>,
{
    type Future = Box<Future<Item = http::Response, Error = http::Error>>;
    fn call(req: http::Request, template: Option<Template>, mut env: Environment) -> Self::Future {
        Box::new(parse_id::<T>(&req).into_future().and_then(move |id| {
            F::Receiver::receive(req, &mut env).into_future().and_then(move |parts| {
                let request = M::Request::new(parts, id, &mut env);
                M::call(request, env).then(move |result| {
                    let presenter = F::Presenter::for_resource();
                    match result {
                        Ok(resource)    => Ok(presenter.resource(resource, template)),
                        Err(error)      => Ok(presenter.error(error, template)),
                    }
                })
            })
        }).or_else(move |err| {
            Ok(F::Presenter::for_resource().error(err, template))
        }))
    }
}

impl<M, T, F> Endpoint<_Collection, _Collection> for (T, F, M)
where
    T: ResourceEndpoint,
    M: ?Sized + Method<T>,
    M::Request: CollectionRequest<T>,
    M::Outcome: Stream<Item = M::Response, Error = Error>,
    F: Format<T, M>,
{
    type Future = Box<Future<Item = http::Response, Error = http::Error>>;
    fn call(req: http::Request, template: Option<Template>, mut env: Environment) -> Self::Future {
        Box::new(F::Receiver::receive(req, &mut env).into_future().and_then(move |parts| {
            let request = M::Request::new(parts, &mut env);
            let stream = M::call(request, env);
            let presenter = F::Presenter::for_collection();
            stream.then(move |result| -> Result<_, ()> { Ok(result) }).fold(presenter, move |mut presenter, result| {
                match result {
                    Ok(resource)    => presenter.append(resource, template),
                    Err(error)      => presenter.error(error, template),
                }
                Ok(presenter)
            }).then(|presenter| Ok(presenter.unwrap().finish()))
        }).or_else(move |err| {
            Ok(F::Presenter::for_resource().error(err, template))
        }))
    }
}

impl<M, T, F> Endpoint<_Collection, _Resource> for (T, F, M)
where
    T: ResourceEndpoint,
    M: ?Sized + Method<T>,
    M::Request: CollectionRequest<T>,
    M::Outcome: Future<Item = M::Response, Error = Error>,
    F: Format<T, M>,
{
    type Future = Box<Future<Item = http::Response, Error = http::Error>>;
    fn call(req: http::Request, template: Option<Template>, mut env: Environment) -> Self::Future {
        Box::new(F::Receiver::receive(req, &mut env).into_future().and_then(move |parts| {
            let request = M::Request::new(parts, &mut env);
            M::call(request, env).then(move |result| {
                let presenter = F::Presenter::for_resource();
                match result {
                    Ok(resource)    => Ok(presenter.resource(resource, template)),
                    Err(error)      => Ok(presenter.error(error, template)),
                }
            })
        }).or_else(move |err| {
            Ok(F::Presenter::for_resource().error(err, template))
        }))
    }
}

impl<M, T, F> Endpoint<_Resource, _Collection> for (T, F, M)
where
    T: ResourceEndpoint,
    M: ?Sized + Method<T>,
    M::Request: ResourceRequest<T>,
    M::Outcome: Stream<Item = M::Response, Error = Error>,
    F: Format<T, M>,
{
    type Future = Box<Future<Item = http::Response, Error = http::Error>>;
    fn call(req: http::Request, template: Option<Template>, mut env: Environment) -> Self::Future {
        Box::new(parse_id::<T>(&req).into_future().and_then(move |id| {
            F::Receiver::receive(req, &mut env).into_future().and_then(move |parts| {
                let request = M::Request::new(parts, id, &mut env);
                let stream = M::call(request, env);
                let presenter = F::Presenter::for_collection();
                stream.then(move |result| -> Result<_, ()> { Ok(result) }).fold(presenter, move |mut presenter, result| {
                    match result {
                        Ok(resource)    => presenter.append(resource, template),
                        Err(error)      => presenter.error(error, template),
                    }
                    Ok(presenter)
                }).then(|presenter| Ok(presenter.unwrap().finish()))
            })
        }).or_else(move |err| {
            Ok(F::Presenter::for_resource().error(err, template))
        }))
    }
}

pub fn endpoint<H, R, E: ?Sized + Endpoint<H, R>>(req: http::Request, template: Option<Template>, env: Environment)
    -> E::Future
{
    E::call(req, template, env)
}
