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
    fn call(req: Request, template: Option<Template>) -> http::BoxFuture;
}

fn parse_id<T: ResourceEndpoint>(id: Option<String>) -> Result<T::Identifier, Error> {
    id.ok_or(Error).and_then(|id| id.parse().or(Err(Error)))
}

pub struct _Resource;

pub struct _Collection;

pub struct _Unit;

impl<M, T, F, R> Endpoint<_Resource, _Resource> for (T, F, M)
where
    T: ResourceEndpoint,
    M: ?Sized + ResourceMethod<T, Response = R>,
    R: ResourceEndpoint,
    F: Format<T, R, M>,
{
    fn call(req: Request, template: Option<Template>) -> http::BoxFuture {
        let Request { req, id, mut env } = req;
        let id = match parse_id::<T>(id) {
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
    F: Format<T, (), M>,
{
    fn call(req: Request, template: Option<Template>) -> http::BoxFuture {
        let Request { req, id, mut env } = req;
        let id = match parse_id::<T>(id) {
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

impl<M, T, F, R> Endpoint<_Resource, _Collection> for (T, F, M)
where
    T: ResourceEndpoint,
    M: ?Sized + ResourceMethod<T, Response = Vec<R>>,
    R: ResourceEndpoint,
    F: Format<T, R, M>,
{
    fn call(req: Request, template: Option<Template>) -> http::BoxFuture {
        let Request { req, id, mut env } = req;
        let id = match parse_id::<T>(id) {
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

impl<M, T, F, R> Endpoint<_Collection, _Resource> for (T, F, M)
where
    T: ResourceEndpoint,
    M: ?Sized + CollectionMethod<T, Response = R>,
    R: ResourceEndpoint,
    F: Format<T, R, M>,
{
    fn call(req: Request, template: Option<Template>) -> http::BoxFuture {
        let Request { req, mut env, .. } = req;
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
    F: Format<T, (), M>,
{
    fn call(req: Request, template: Option<Template>) -> http::BoxFuture {
        let Request { req, mut env, .. } = req;
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

impl<M, T, F, R> Endpoint<_Collection, _Collection> for (T, F, M)
where
    T: ResourceEndpoint,
    M: ?Sized + CollectionMethod<T, Response = Vec<R>>,
    R: ResourceEndpoint,
    F: Format<T, R, M>,
{
    fn call(req: Request, template: Option<Template>) -> http::BoxFuture {
        let Request { req, mut env, .. } = req;
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
    pub id: Option<String>,
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
        T::call(req, self.template)
    }
}
