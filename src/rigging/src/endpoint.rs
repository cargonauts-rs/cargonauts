use futures::{Future, Stream};

use Error;
use ResourceEndpoint;
use method::Method;

use request::*;
use format::*;
use environment::Environment;
use http;

pub trait Endpoint<Hits, Returns> {
    fn call(http::Request, template: Option<Template>, env: Environment) -> http::BoxFuture;
}

fn parse_id<T: ResourceEndpoint>(req: &http::Request) -> Result<T::Identifier, Error> {
    req.path().split('/').filter(|p| !p.is_empty()).skip(1).next().ok_or(Error).and_then(|s| {
        s.parse().map_err(|_| Error)
    })
}

pub struct _Resource;

pub struct _Collection;

pub struct _Unit;

impl<M, T, F> Endpoint<_Resource, _Resource> for (T, F, M)
where
    T: ResourceEndpoint,
    M: ?Sized + Method<T>,
    M::Request: ResourceRequest<T>,
    M::Response: ResourceEndpoint,
    M::Outcome: Future<Item = M::Response, Error = Error>,
    F: Format<T, M>,
{
    fn call(req: http::Request, template: Option<Template>, env: Environment) -> http::BoxFuture {
        let id = match parse_id::<T>(&req) {
            Ok(id) => id,
            Err(err) => return F::Presenter::error(err, &env),
        };
        let parts = match F::Receiver::receive(req, &env) {
            Ok(parts)   => parts,
            Err(err)    => return F::Presenter::error(err, &env),
        };
        let request = M::Request::new(parts, id, &env);
        let future = M::call(request, &env);
        F::Presenter::resource(future, template, &env)
    }
}

impl<M, T, F> Endpoint<_Resource, _Unit> for (T, F, M)
where
    T: ResourceEndpoint,
    M: ?Sized + Method<T, Response = ()>,
    M::Request: ResourceRequest<T>,
    M::Outcome: Future<Item = M::Response, Error = Error>,
    F: Format<T, M>,
{
    fn call(req: http::Request, template: Option<Template>, env: Environment) -> http::BoxFuture {
        let id = match parse_id::<T>(&req) {
            Ok(id) => id,
            Err(err) => return F::Presenter::error(err, &env),
        };
        let parts = match F::Receiver::receive(req, &env) {
            Ok(parts)   => parts,
            Err(err)    => return F::Presenter::error(err, &env),
        };
        let request = M::Request::new(parts, id, &env);
        let future = M::call(request, &env);
        F::Presenter::unit(future, template, &env)
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
    fn call(req: http::Request, template: Option<Template>, env: Environment) -> http::BoxFuture {
        let id = match parse_id::<T>(&req) {
            Ok(id) => id,
            Err(err) => return F::Presenter::error(err, &env),
        };
        let parts = match F::Receiver::receive(req, &env) {
            Ok(parts)   => parts,
            Err(err)    => return F::Presenter::error(err, &env),
        };
        let request = M::Request::new(parts, id, &env);
        let stream = M::call(request, &env);
        F::Presenter::collection(stream, template, &env)
    }
}

impl<M, T, F> Endpoint<_Collection, _Resource> for (T, F, M)
where
    T: ResourceEndpoint,
    M: ?Sized + Method<T>,
    M::Request: CollectionRequest<T>,
    M::Response: ResourceEndpoint,
    M::Outcome: Future<Item = M::Response, Error = Error>,
    F: Format<T, M>,
{
    fn call(req: http::Request, template: Option<Template>, env: Environment) -> http::BoxFuture {
        let parts = match F::Receiver::receive(req, &env) {
            Ok(parts)   => parts,
            Err(err)    => return F::Presenter::error(err, &env),
        };
        let request = M::Request::new(parts, &env);
        let future = M::call(request, &env);
        F::Presenter::resource(future, template, &env)
    }
}

impl<M, T, F> Endpoint<_Collection, _Unit> for (T, F, M)
where
    T: ResourceEndpoint,
    M: ?Sized + Method<T, Response = ()>,
    M::Request: CollectionRequest<T>,
    M::Response: ResourceEndpoint,
    M::Outcome: Future<Item = M::Response, Error = Error>,
    F: Format<T, M>,
{
    fn call(req: http::Request, template: Option<Template>, env: Environment) -> http::BoxFuture {
        let parts = match F::Receiver::receive(req, &env) {
            Ok(parts)   => parts,
            Err(err)    => return F::Presenter::error(err, &env),
        };
        let request = M::Request::new(parts, &env);
        let future = M::call(request, &env);
        F::Presenter::unit(future, template, &env)
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
    fn call(req: http::Request, template: Option<Template>, env: Environment) -> http::BoxFuture {
        let parts = match F::Receiver::receive(req, &env) {
            Ok(parts)   => parts,
            Err(err)    => return F::Presenter::error(err, &env),
        };
        let request = M::Request::new(parts, &env);
        let stream = M::call(request, &env);
        F::Presenter::collection(stream, template, &env)
    }
}


pub fn endpoint<H, R, E: ?Sized + Endpoint<H, R>>(req: http::Request, template: Option<Template>, env: Environment)
    -> http::BoxFuture
{
    E::call(req, template, env)
}
