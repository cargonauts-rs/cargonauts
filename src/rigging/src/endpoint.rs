use std::marker::PhantomData;
use std::rc::Rc;

use futures::Future;
use tokio::Service;

use Error;
use resource::ResourceEndpoint;
use method::*;

use format::{Format, TemplateKey};
use environment::Environment;
use http::{self, StatusCode};

pub trait Endpoint<F, M: ?Sized, In, Out> {
    fn call(req: Request, format: Rc<F>, key: TemplateKey) -> http::BoxFuture;
}

fn parse_id<T: ResourceEndpoint>(id: Option<String>) -> Result<T::Identifier, Error> {
    match id {
        Some(id) => id.parse().map_err(|_| Error::with_msg(StatusCode::BadRequest, 
                                                           format!("Could not parse resource identifier: {}", id))),
        None     => Err(Error::with_msg(StatusCode::BadRequest, "Missing resource identifier."))
    }
}

pub struct _Resource;

pub struct _Collection;

pub struct _Unit;

impl<M, T, F, R> Endpoint<F, M, _Resource, _Resource> for T
where
    T: ResourceEndpoint,
    M: ?Sized + ResourceMethod<T, Response = R>,
    R: ResourceEndpoint,
    F: Format<T, R, M>,
{
    fn call(req: Request, format: Rc<F>, key: TemplateKey) -> http::BoxFuture {
        let Request { req, id, env } = req;
        let id = match parse_id::<T>(id) {
            Ok(id) => id,
            Err(err) => return F::present_error(&format, err, &env),
        };
        Box::new(F::receive_request(&format, req, &env).then(move |result| {
            let request = match result {
                Ok(parts)   => parts,
                Err(err)    => return F::present_error(&format, err, &env),
            };
            let future = M::call(id, request, env.clone());
            F::present_resource(&format, future, key, &env)
        }))
    }
}

impl<M, T, F> Endpoint<F, M, _Resource, _Unit> for T
where
    T: ResourceEndpoint,
    M: ?Sized + ResourceMethod<T, Response = ()>,
    F: Format<T, (), M>,
{
    fn call(req: Request, format: Rc<F>, key: TemplateKey) -> http::BoxFuture {
        let Request { req, id, env } = req;
        let id = match parse_id::<T>(id) {
            Ok(id) => id,
            Err(err) => return F::present_error(&format, err, &env),
        };
        Box::new(F::receive_request(&format, req, &env).then(move |result| {
            let request = match result {
                Ok(parts)   => parts,
                Err(err)    => return F::present_error(&format, err, &env),
            };
            let future = M::call(id, request, env.clone());
            F::present_unit(&format, future, key, &env)
        }))
    }
}

impl<M, T, F, R> Endpoint<F, M, _Resource, _Collection> for T
where
    T: ResourceEndpoint,
    M: ?Sized + ResourceMethod<T, Response = Vec<R>>,
    R: ResourceEndpoint,
    F: Format<T, R, M>,
{
    fn call(req: Request, format: Rc<F>, key: TemplateKey) -> http::BoxFuture {
        let Request { req, id, env } = req;
        let id = match parse_id::<T>(id) {
            Ok(id) => id,
            Err(err) => return F::present_error(&format, err, &env),
        };
        Box::new(F::receive_request(&format, req, &env).then(move |result| {
            let request = match result {
                Ok(parts)   => parts,
                Err(err)    => return F::present_error(&format, err, &env),
            };
            let future = M::call(id, request, env.clone());
            F::present_collection(&format, future, key, &env)
        }))
    }
}

impl<M, T, F, R> Endpoint<F, M, _Collection, _Resource> for T
where
    T: ResourceEndpoint,
    M: ?Sized + CollectionMethod<T, Response = R>,
    R: ResourceEndpoint,
    F: Format<T, R, M>,
{
    fn call(req: Request, format: Rc<F>, key: TemplateKey) -> http::BoxFuture {
        let Request { req, env, .. } = req;
        Box::new(F::receive_request(&format, req, &env).then(move |result| {
            let request = match result {
                Ok(parts)   => parts,
                Err(err)    => return F::present_error(&format, err, &env),
            };
            let future = M::call(request, env.clone());
            F::present_resource(&format, future, key, &env)
        }))
    }
}

impl<M, T, F> Endpoint<F, M, _Collection, _Unit> for T
where
    T: ResourceEndpoint,
    M: ?Sized + CollectionMethod<T, Response = ()>,
    F: Format<T, (), M>,
{
    fn call(req: Request, format: Rc<F>, key: TemplateKey) -> http::BoxFuture {
        let Request { req, env, .. } = req;
        Box::new(F::receive_request(&format, req, &env).then(move |result| {
            let request = match result {
                Ok(parts)   => parts,
                Err(err)    => return F::present_error(&format, err, &env),
            };
            let future = M::call(request, env.clone());
            F::present_unit(&format, future, key, &env)
        }))
    }
}

impl<M, T, F, R> Endpoint<F, M, _Collection, _Collection> for T
where
    T: ResourceEndpoint,
    M: ?Sized + CollectionMethod<T, Response = Vec<R>>,
    R: ResourceEndpoint,
    F: Format<T, R, M>,
{
    fn call(req: Request, format: Rc<F>, key: TemplateKey) -> http::BoxFuture {
        let Request { req, env, .. } = req;
        Box::new(F::receive_request(&format, req, &env).then(move |result| {
            let request = match result {
                Ok(parts)   => parts,
                Err(err)    => return F::present_error(&format, err, &env),
            };
            let future = M::call(request, env.clone());
            F::present_collection(&format, future, key, &env)
        }))
    }
}

pub struct Request {
    pub req: http::Request,
    pub env: Environment,
    pub id: Option<String>,
}

pub struct EndpointService<I, O, T, F, M: ?Sized> {
    key: TemplateKey,
    format: Rc<F>,
    _marker: PhantomData<(I, O, T, M)>
}

impl<I, O, T, F, M: ?Sized> EndpointService<I, O, T, F, M> {
    #[doc(hidden)]
    pub fn new(key: TemplateKey, format: Rc<F>) -> Self {
        Self { key, format, _marker: PhantomData }
    }
}

impl<I, O, T, F, M> Service for EndpointService<I, O, T, F, M>
where
    T: Endpoint<F, M, I, O>,
    M: ?Sized
{
    type Request = Request;
    type Response = http::Response;
    type Error = http::Error;
    type Future = http::BoxFuture;

    fn call(&self, req: Self::Request) -> Self::Future {
        T::call(req, self.format.clone(), self.key)
    }
}
