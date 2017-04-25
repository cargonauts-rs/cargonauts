use std::fmt::Debug;

use rigging::{ResourceEndpoint, Error};
use rigging::environment::Environment;
use rigging::http;
use rigging::format::*;
use rigging::method::Method;
use rigging::request::Request;

const MIME: &'static str = "ext/plain; charset=utf-8";

#[derive(Default, Clone)]
pub struct SimpleDebug {
    _private: (),
}

impl<T, M> Format<T, M> for SimpleDebug
where
    T: ResourceEndpoint,
    M: ?Sized + Method<T>,
    M::Request: Request<T, BodyParts = ()>,
    M::Response: Debug,
{
    type Presenter = Self;
    type Receiver = Self;
}

impl<T, M> Receive<T, M> for SimpleDebug
where
    T: ResourceEndpoint,
    M: ?Sized + Method<T>,
    M::Request: Request<T, BodyParts = ()>,
{
    fn receive(_: http::Request, _: &mut Environment) -> Result<<M::Request as Request<T>>::BodyParts, Error> {
        Ok(())
    }
}


impl<T, M> Present<T, M> for SimpleDebug
where
    T: ResourceEndpoint,
    M: ?Sized + Method<T>,
    M::Response: Debug,
{
    type ResourcePresenter = ResourcePresenter;
    type CollectionPresenter = CollectionPresenter<M::Response>;

    fn for_resource() -> Self::ResourcePresenter {
        ResourcePresenter::default()
    }

    fn for_collection() -> Self::CollectionPresenter {
        CollectionPresenter::default()
    }
}

#[derive(Default, Clone, Copy)]
pub struct ResourcePresenter;

impl<T, M> PresentResource<T, M> for ResourcePresenter
where
    T: ResourceEndpoint,
    M: ?Sized + Method<T>,
    M::Response: Debug,
{
    fn resource(self, resource: M::Response, _: Option<Template>)
        -> http::Response
    {
        debug_response(resource, http::StatusCode::Ok)
    }

    fn error(self, error: Error, _: Option<Template>)
        -> http::Response
    {
        debug_response(error, http::StatusCode::InternalServerError)
    }
}

pub struct CollectionPresenter<T> {
    resources: Result<Vec<T>, Error>,
}

impl<T> Default for CollectionPresenter<T> {
    fn default() -> Self {
        Self { resources: Ok(vec![]) }
    }
}

impl<T> Clone for CollectionPresenter<T> {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl<T, M> PresentCollection<T, M> for CollectionPresenter<M::Response>
where
    T: ResourceEndpoint,
    M: ?Sized + Method<T>,
    M::Response: Debug,
{
    fn append(&mut self, resource: M::Response, _: Option<Template>) {
        let _ = self.resources.as_mut().map(|v| v.push(resource));
    }

    fn error(&mut self, error: Error, _: Option<Template>) {
        self.resources = Err(error);
    }

    fn finish(self) -> http::Response {
        match self.resources {
            Ok(resources)   => debug_response(resources, http::StatusCode::Ok),
            Err(error)      => debug_response(error, http::StatusCode::InternalServerError),
        }
    }
}

fn debug_response<T: Debug>(t: T, status: http::StatusCode) -> http::Response {
    let data = format!("{:?}", t);
    http::Response::new().with_status(status)
                         .with_header(http::headers::ContentLength(data.len() as u64))
                         .with_header(http::headers::ContentType(MIME.parse().unwrap()))
                         .with_body(data)
}
