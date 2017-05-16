use std::marker::PhantomData;
use std::rc::Rc;

use futures::Future;
use tokio::Service;

use Error;
use resource::ResourceEndpoint;
use method::*;

use format::Format;
use environment::Environment;
use http;

pub trait Endpoint<F, In, Out> {
    fn call(req: Request, format: Rc<F>, path: &'static str) -> http::BoxFuture;
}

fn parse_id<T: ResourceEndpoint>(id: Option<String>) -> Result<T::Identifier, Error> {
    id.ok_or(Error).and_then(|id| id.parse().or(Err(Error)))
}

pub struct _Resource;

pub struct _Collection;

pub struct _Unit;

impl<M, T, F, R> Endpoint<F, _Resource, _Resource> for (T, M)
where
    T: ResourceEndpoint,
    M: ?Sized + ResourceMethod<T, Response = R>,
    R: ResourceEndpoint,
    F: Format<T, R, M>,
{
    fn call(req: Request, format: Rc<F>, path: &'static str) -> http::BoxFuture {
        let Request { req, id, mut env } = req;
        let id = match parse_id::<T>(id) {
            Ok(id) => id,
            Err(err) => return F::present_error(&format, err, &mut env),
        };
        Box::new(F::receive_request(&format, req, &mut env).then(move |result| {
            let request = match result {
                Ok(parts)   => parts,
                Err(err)    => return F::present_error(&format, err, &mut env),
            };
            let future = M::call(id, request, &mut env);
            F::present_resource(&format, future, path, &mut env)
        }))
    }
}

impl<M, T, F> Endpoint<F, _Resource, _Unit> for (T, M)
where
    T: ResourceEndpoint,
    M: ?Sized + ResourceMethod<T, Response = ()>,
    F: Format<T, (), M>,
{
    fn call(req: Request, format: Rc<F>, path: &'static str) -> http::BoxFuture {
        let Request { req, id, mut env } = req;
        let id = match parse_id::<T>(id) {
            Ok(id) => id,
            Err(err) => return F::present_error(&format, err, &mut env),
        };
        Box::new(F::receive_request(&format, req, &mut env).then(move |result| {
            let request = match result {
                Ok(parts)   => parts,
                Err(err)    => return F::present_error(&format, err, &mut env),
            };
            let future = M::call(id, request, &mut env);
            F::present_unit(&format, future, path, &mut env)
        }))
    }
}

impl<M, T, F, R> Endpoint<F, _Resource, _Collection> for (T, M)
where
    T: ResourceEndpoint,
    M: ?Sized + ResourceMethod<T, Response = Vec<R>>,
    R: ResourceEndpoint,
    F: Format<T, R, M>,
{
    fn call(req: Request, format: Rc<F>, path: &'static str) -> http::BoxFuture {
        let Request { req, id, mut env } = req;
        let id = match parse_id::<T>(id) {
            Ok(id) => id,
            Err(err) => return F::present_error(&format, err, &mut env),
        };
        Box::new(F::receive_request(&format, req, &mut env).then(move |result| {
            let request = match result {
                Ok(parts)   => parts,
                Err(err)    => return F::present_error(&format, err, &mut env),
            };
            let future = M::call(id, request, &mut env);
            F::present_collection(&format, future, path, &mut env)
        }))
    }
}

impl<M, T, F, R> Endpoint<F, _Collection, _Resource> for (T, M)
where
    T: ResourceEndpoint,
    M: ?Sized + CollectionMethod<T, Response = R>,
    R: ResourceEndpoint,
    F: Format<T, R, M>,
{
    fn call(req: Request, format: Rc<F>, path: &'static str) -> http::BoxFuture {
        let Request { req, mut env, .. } = req;
        Box::new(F::receive_request(&format, req, &mut env).then(move |result| {
            let request = match result {
                Ok(parts)   => parts,
                Err(err)    => return F::present_error(&format, err, &mut env),
            };
            let future = M::call(request, &mut env);
            F::present_resource(&format, future, path, &mut env)
        }))
    }
}

impl<M, T, F> Endpoint<F, _Collection, _Unit> for (T, M)
where
    T: ResourceEndpoint,
    M: ?Sized + CollectionMethod<T, Response = ()>,
    F: Format<T, (), M>,
{
    fn call(req: Request, format: Rc<F>, path: &'static str) -> http::BoxFuture {
        let Request { req, mut env, .. } = req;
        Box::new(F::receive_request(&format, req, &mut env).then(move |result| {
            let request = match result {
                Ok(parts)   => parts,
                Err(err)    => return F::present_error(&format, err, &mut env),
            };
            let future = M::call(request, &mut env);
            F::present_unit(&format, future, path, &mut env)
        }))
    }
}

impl<M, T, F, R> Endpoint<F, _Collection, _Collection> for (T, M)
where
    T: ResourceEndpoint,
    M: ?Sized + CollectionMethod<T, Response = Vec<R>>,
    R: ResourceEndpoint,
    F: Format<T, R, M>,
{
    fn call(req: Request, format: Rc<F>, path: &'static str) -> http::BoxFuture {
        let Request { req, mut env, .. } = req;
        Box::new(F::receive_request(&format, req, &mut env).then(move |result| {
            let request = match result {
                Ok(parts)   => parts,
                Err(err)    => return F::present_error(&format, err, &mut env),
            };
            let future = M::call(request, &mut env);
            F::present_collection(&format, future, path, &mut env)
        }))
    }
}

pub struct Request {
    pub req: http::Request,
    pub env: Environment,
    pub id: Option<String>,
}

pub struct EndpointService<I, O, T: ?Sized, F> {
    path: &'static str,
    format: Rc<F>,
    _marker: PhantomData<(I, O, T)>
}

impl<I, O, T: ?Sized, F> EndpointService<I, O, T, F> {
    #[doc(hidden)]
    pub fn new(path: &'static str, format: Rc<F>) -> Self {
        Self { path, format, _marker: PhantomData }
    }
}

impl<I, O, T, F> Service for EndpointService<I, O, T, F>
where
    T: ?Sized + Endpoint<F, I, O>,
{
    type Request = Request;
    type Response = http::Response;
    type Error = http::Error;
    type Future = http::BoxFuture;

    fn call(&self, req: Self::Request) -> Self::Future {
        T::call(req, self.format.clone(), self.path)
    }
}
