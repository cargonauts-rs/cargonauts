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
    fn call(req: http::Request, template: Option<Template>, mut env: Environment) -> http::BoxFuture {
        let id = match parse_id::<T>(&req) {
            Ok(id) => id,
            Err(err) => return F::present_error(err, &mut env),
        };
        Box::new(F::receive_request(req, &mut env).then(move |result| {
            let parts = match result {
                Ok(parts)   => parts,
                Err(err)    => return F::present_error(err, &mut env),
            };
            let request = M::Request::new(parts, id, &mut env);
            let future = M::call(request, &mut env);
            F::present_resource(future, template, &mut env)
        }))
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
    fn call(req: http::Request, template: Option<Template>, mut env: Environment) -> http::BoxFuture {
        let id = match parse_id::<T>(&req) {
            Ok(id) => id,
            Err(err) => return F::present_error(err, &mut env),
        };
        Box::new(F::receive_request(req, &mut env).then(move |result| {
            let parts = match result {
                Ok(parts)   => parts,
                Err(err)    => return F::present_error(err, &mut env),
            };
            let request = M::Request::new(parts, id, &mut env);
            let future = M::call(request, &mut env);
            F::present_unit(future, template, &mut env)
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
    fn call(req: http::Request, template: Option<Template>, mut env: Environment) -> http::BoxFuture {
        let id = match parse_id::<T>(&req) {
            Ok(id) => id,
            Err(err) => return F::present_error(err, &mut env),
        };
        Box::new(F::receive_request(req, &mut env).then(move |result| {
            let parts = match result {
                Ok(parts)   => parts,
                Err(err)    => return F::present_error(err, &mut env),
            };
            let request = M::Request::new(parts, id, &mut env);
            let stream = M::call(request, &mut env);
            F::present_collection(stream, template, &mut env)
        }))
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
    fn call(req: http::Request, template: Option<Template>, mut env: Environment) -> http::BoxFuture {
        Box::new(F::receive_request(req, &mut env).then(move |result| {
            let parts = match result {
                Ok(parts)   => parts,
                Err(err)    => return F::present_error(err, &mut env),
            };
            let request = M::Request::new(parts, &mut env);
            let future = M::call(request, &mut env);
            F::present_resource(future, template, &mut env)
        }))
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
    fn call(req: http::Request, template: Option<Template>, mut env: Environment) -> http::BoxFuture {
        Box::new(F::receive_request(req, &mut env).then(move |result| {
            let parts = match result {
                Ok(parts)   => parts,
                Err(err)    => return F::present_error(err, &mut env),
            };
            let request = M::Request::new(parts, &mut env);
            let future = M::call(request, &mut env);
            F::present_unit(future, template, &mut env)
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
    fn call(req: http::Request, template: Option<Template>, mut env: Environment) -> http::BoxFuture {
        Box::new(F::receive_request(req, &mut env).then(move |result| {
            let parts = match result {
                Ok(parts)   => parts,
                Err(err)    => return F::present_error(err, &mut env),
            };
            let request = M::Request::new(parts, &mut env);
            let stream = M::call(request, &mut env);
            F::present_collection(stream, template, &mut env)
        }))
    }
}


pub fn endpoint<H, R, E: ?Sized + Endpoint<H, R>>(req: http::Request, template: Option<Template>, env: Environment)
    -> http::BoxFuture
{
    E::call(req, template, env)
}
