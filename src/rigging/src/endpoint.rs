use std::marker::PhantomData;

use futures::{Future};
use tokio::Service;

use Error;
use resource::ResourceEndpoint;
use method::*;

use format::*;
use environment::Environment;
use http;

pub trait Endpoint<In, Out> {
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
    M: ?Sized + ResourceMethod<T>,
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
            let request = match result {
                Ok(parts)   => parts,
                Err(err)    => return F::present_error(err, &mut env),
            };
            let future = M::call(id, request, &mut env);
            F::present_resource(future, template, &mut env)
        }))
    }
}

impl<M, T, F> Endpoint<_Resource, _Unit> for (T, F, M)
where
    T: ResourceEndpoint,
    M: ?Sized + ResourceMethod<T, Response = ()>,
    M::Outcome: Future<Item = M::Response, Error = Error>,
    F: Format<T, M>,
{
    fn call(req: http::Request, template: Option<Template>, mut env: Environment) -> http::BoxFuture {
        let id = match parse_id::<T>(&req) {
            Ok(id) => id,
            Err(err) => return F::present_error(err, &mut env),
        };
        Box::new(F::receive_request(req, &mut env).then(move |result| {
            let request = match result {
                Ok(parts)   => parts,
                Err(err)    => return F::present_error(err, &mut env),
            };
            let future = M::call(id, request, &mut env);
            F::present_unit(future, template, &mut env)
        }))
    }
}

impl<M, T, F> Endpoint<_Resource, _Collection> for (T, F, M)
where
    T: ResourceEndpoint,
    M: ?Sized + ResourceMethod<T>,
    M::Outcome: Future<Item = Vec<M::Response>, Error = Error>,
    F: Format<T, M>,
{
    fn call(req: http::Request, template: Option<Template>, mut env: Environment) -> http::BoxFuture {
        let id = match parse_id::<T>(&req) {
            Ok(id) => id,
            Err(err) => return F::present_error(err, &mut env),
        };
        Box::new(F::receive_request(req, &mut env).then(move |result| {
            let request = match result {
                Ok(parts)   => parts,
                Err(err)    => return F::present_error(err, &mut env),
            };
            let future = M::call(id, request, &mut env);
            F::present_collection(future, template, &mut env)
        }))
    }
}

impl<M, T, F> Endpoint<_Collection, _Resource> for (T, F, M)
where
    T: ResourceEndpoint,
    M: ?Sized + CollectionMethod<T>,
    M::Response: ResourceEndpoint,
    M::Outcome: Future<Item = M::Response, Error = Error>,
    F: Format<T, M>,
{
    fn call(req: http::Request, template: Option<Template>, mut env: Environment) -> http::BoxFuture {
        Box::new(F::receive_request(req, &mut env).then(move |result| {
            let request = match result {
                Ok(parts)   => parts,
                Err(err)    => return F::present_error(err, &mut env),
            };
            let future = M::call(request, &mut env);
            F::present_resource(future, template, &mut env)
        }))
    }
}

impl<M, T, F> Endpoint<_Collection, _Unit> for (T, F, M)
where
    T: ResourceEndpoint,
    M: ?Sized + CollectionMethod<T, Response = ()>,
    M::Response: ResourceEndpoint,
    M::Outcome: Future<Item = M::Response, Error = Error>,
    F: Format<T, M>,
{
    fn call(req: http::Request, template: Option<Template>, mut env: Environment) -> http::BoxFuture {
        Box::new(F::receive_request(req, &mut env).then(move |result| {
            let request = match result {
                Ok(parts)   => parts,
                Err(err)    => return F::present_error(err, &mut env),
            };
            let future = M::call(request, &mut env);
            F::present_unit(future, template, &mut env)
        }))
    }
}

impl<M, T, F> Endpoint<_Collection, _Collection> for (T, F, M)
where
    T: ResourceEndpoint,
    M: ?Sized + CollectionMethod<T>,
    M::Outcome: Future<Item = Vec<M::Response>, Error = Error>,
    F: Format<T, M>,
{
    fn call(req: http::Request, template: Option<Template>, mut env: Environment) -> http::BoxFuture {
        Box::new(F::receive_request(req, &mut env).then(move |result| {
            let request = match result {
                Ok(parts)   => parts,
                Err(err)    => return F::present_error(err, &mut env),
            };
            let future = M::call(request, &mut env);
            F::present_collection(future, template, &mut env)
        }))
    }
}

pub struct Request {
    pub req: http::Request,
    pub env: Environment,
}

pub struct EndpointService<I, O, T: ?Sized> {
    template: Option<Template>,
    _marker: PhantomData<(I, O, T)>
}

impl<I, O, T: ?Sized> EndpointService<I, O, T> {
    #[doc(hidden)]
    pub fn new(template: Option<Template>) -> Self {
        Self { template, _marker: PhantomData }
    }
}

impl<I, O, T> Service for EndpointService<I, O, T>
where
    T: ?Sized + Endpoint<I, O>,
{
    type Request = Request;
    type Response = http::Response;
    type Error = http::Error;
    type Future = http::BoxFuture;

    fn call(&self, req: Self::Request) -> Self::Future {
        T::call(req.req, self.template, req.env)
    }
}
