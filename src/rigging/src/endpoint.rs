use std::io;
use std::marker::PhantomData;
use std::rc::Rc;
use std::time::Duration;

use core::reactor::Timeout;
use futures::Future;
use tokio::Service;

use Error;
use environment::Environment;
use format::{Format, TemplateKey};
use http::{self, StatusCode};
use method::*;
use resource::ResourceEndpoint;

pub trait Endpoint<F: Format<Self, R, M>, M: ?Sized + Method<Self>, R, In, Out>: ResourceEndpoint {
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

impl<M, T, F, R> Endpoint<F, M, R, _Resource, _Resource> for T
where
    T: ResourceEndpoint,
    M: ?Sized + ResourceMethod<T, Response = R>,
    R: ResourceEndpoint,
    F: Format<T, R, M>,
{
    fn call(req: Request, format: Rc<F>, key: TemplateKey) -> http::BoxFuture {
        let Request { req, id, env, timeout, duration, .. } = req;
        let timeout = match timeout {
            Ok(timeout) => {
                let env = env.clone();
                let format = format.clone();
                timeout.then(move |_| F::present_error(&format, Error::timeout(duration), &env))
            }
            Err(err)    => return F::present_error(&format, err.into(), &env),
        };
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
        }).select(timeout).then(|result| match result {
            Ok((ok, _))     => Ok(ok),
            Err((err, _))   => Err(err),
        }))
    }
}

impl<M, T, F> Endpoint<F, M, (), _Resource, _Unit> for T
where
    T: ResourceEndpoint,
    M: ?Sized + ResourceMethod<T, Response = ()>,
    F: Format<T, (), M>,
{
    fn call(req: Request, format: Rc<F>, key: TemplateKey) -> http::BoxFuture {
        let Request { req, id, env, timeout, duration, .. } = req;
        let id = match parse_id::<T>(id) {
            Ok(id) => id,
            Err(err) => return F::present_error(&format, err, &env),
        };
        let timeout = match timeout {
            Ok(timeout) => {
                let env = env.clone();
                let format = format.clone();
                timeout.then(move |_| F::present_error(&format, Error::timeout(duration), &env))
            }
            Err(err)    => return F::present_error(&format, err.into(), &env),
        };
        Box::new(F::receive_request(&format, req, &env).then(move |result| {
            let request = match result {
                Ok(parts)   => parts,
                Err(err)    => return F::present_error(&format, err, &env),
            };
            let future = M::call(id, request, env.clone());
            F::present_unit(&format, future, key, &env)
        }).select(timeout).then(|result| match result {
            Ok((ok, _))     => Ok(ok),
            Err((err, _))   => Err(err),
        }))
    }
}

impl<M, T, F, R> Endpoint<F, M, R, _Resource, _Collection> for T
where
    T: ResourceEndpoint,
    M: ?Sized + ResourceMethod<T, Response = Vec<R>>,
    R: ResourceEndpoint,
    F: Format<T, R, M>,
{
    fn call(req: Request, format: Rc<F>, key: TemplateKey) -> http::BoxFuture {
        let Request { req, id, env, timeout, duration, .. } = req;
        let id = match parse_id::<T>(id) {
            Ok(id) => id,
            Err(err) => return F::present_error(&format, err, &env),
        };
        let timeout = match timeout {
            Ok(timeout) => {
                let env = env.clone();
                let format = format.clone();
                timeout.then(move |_| F::present_error(&format, Error::timeout(duration), &env))
            }
            Err(err)    => return F::present_error(&format, err.into(), &env),
        };
        Box::new(F::receive_request(&format, req, &env).then(move |result| {
            let request = match result {
                Ok(parts)   => parts,
                Err(err)    => return F::present_error(&format, err, &env),
            };
            let future = M::call(id, request, env.clone());
            F::present_collection(&format, future, key, &env)
        }).select(timeout).then(|result| match result {
            Ok((ok, _))     => Ok(ok),
            Err((err, _))   => Err(err),
        }))
    }
}

impl<M, T, F, R> Endpoint<F, M, R, _Collection, _Resource> for T
where
    T: ResourceEndpoint,
    M: ?Sized + CollectionMethod<T, Response = R>,
    R: ResourceEndpoint,
    F: Format<T, R, M>,
{
    fn call(req: Request, format: Rc<F>, key: TemplateKey) -> http::BoxFuture {
        let Request { req, env, timeout, duration, .. } = req;
        let timeout = match timeout {
            Ok(timeout) => {
                let env = env.clone();
                let format = format.clone();
                timeout.then(move |_| F::present_error(&format, Error::timeout(duration), &env))
            }
            Err(err)    => return F::present_error(&format, err.into(), &env),
        };
        Box::new(F::receive_request(&format, req, &env).then(move |result| {
            let request = match result {
                Ok(parts)   => parts,
                Err(err)    => return F::present_error(&format, err, &env),
            };
            let future = M::call(request, env.clone());
            F::present_resource(&format, future, key, &env)
        }).select(timeout).then(|result| match result {
            Ok((ok, _))     => Ok(ok),
            Err((err, _))   => Err(err),
        }))
    }
}

impl<M, T, F> Endpoint<F, M, (), _Collection, _Unit> for T
where
    T: ResourceEndpoint,
    M: ?Sized + CollectionMethod<T, Response = ()>,
    F: Format<T, (), M>,
{
    fn call(req: Request, format: Rc<F>, key: TemplateKey) -> http::BoxFuture {
        let Request { req, env, timeout, duration, .. } = req;
        let timeout = match timeout {
            Ok(timeout) => {
                let env = env.clone();
                let format = format.clone();
                timeout.then(move |_| F::present_error(&format, Error::timeout(duration), &env))
            }
            Err(err)    => return F::present_error(&format, err.into(), &env),
        };
        Box::new(F::receive_request(&format, req, &env).then(move |result| {
            let request = match result {
                Ok(parts)   => parts,
                Err(err)    => return F::present_error(&format, err, &env),
            };
            let future = M::call(request, env.clone());
            F::present_unit(&format, future, key, &env)
        }).select(timeout).then(|result| match result {
            Ok((ok, _))     => Ok(ok),
            Err((err, _))   => Err(err),
        }))
    }
}

impl<M, T, F, R> Endpoint<F, M, R, _Collection, _Collection> for T
where
    T: ResourceEndpoint,
    M: ?Sized + CollectionMethod<T, Response = Vec<R>>,
    R: ResourceEndpoint,
    F: Format<T, R, M>,
{
    fn call(req: Request, format: Rc<F>, key: TemplateKey) -> http::BoxFuture {
        let Request { req, env, timeout, duration, .. } = req;
        let timeout = match timeout {
            Ok(timeout) => {
                let env = env.clone();
                let format = format.clone();
                timeout.then(move |_| F::present_error(&format, Error::timeout(duration), &env))
            }
            Err(err)    => return F::present_error(&format, err.into(), &env),
        };
        Box::new(F::receive_request(&format, req, &env).then(move |result| {
            let request = match result {
                Ok(parts)   => parts,
                Err(err)    => return F::present_error(&format, err, &env),
            };
            let future = M::call(request, env.clone());
            F::present_collection(&format, future, key, &env)
        }).select(timeout).then(|result| match result {
            Ok((ok, _))     => Ok(ok),
            Err((err, _))   => Err(err),
        }))
    }
}

pub struct Request {
    pub req: http::Request,
    pub env: Environment,
    pub(crate) id: Option<String>,
    pub(crate) timeout: io::Result<Timeout>,
    pub(crate) duration: Duration,
}

pub struct EndpointService<I, O, T, F, M: ?Sized, R> {
    key: TemplateKey,
    format: Rc<F>,
    _marker: PhantomData<(R, I, O, T, M)>
}

impl<I, O, T, F, M: ?Sized, R> EndpointService<I, O, T, F, M, R> {
    #[doc(hidden)]
    pub fn new(key: TemplateKey, format: Rc<F>) -> Self {
        Self { key, format, _marker: PhantomData }
    }
}

impl<I, O, T, F, M, R> Service for EndpointService<I, O, T, F, M, R>
where
    T: Endpoint<F, M, R, I, O> + ResourceEndpoint,
    M: ?Sized + Method<T>,
    F: Format<T, R, M>,
{
    type Request = Request;
    type Response = http::Response;
    type Error = http::Error;
    type Future = http::BoxFuture;

    fn call(&self, req: Self::Request) -> Self::Future {
        T::call(req, self.format.clone(), self.key)
    }
}
