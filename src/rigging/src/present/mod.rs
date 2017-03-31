pub mod middleware;

use tokio::NewService;
use tokio::stream::NewStreamService;

use mainsail::{Resource, Error};
use request::{ResourceRequest, CollectionRequest};
use http;

pub trait Present<T: Resource>: Default {
    type ResourcePresenter: PresentResource<T>;
    type CollectionPresenter: PresentCollection<T>;

    fn for_resource(&self) -> Self::ResourcePresenter;
    fn for_collection(&self) -> Self::CollectionPresenter;
}

pub trait PresentResource<T: Resource>: Clone + Send + 'static {
    fn resource<R>(self, resource: T, template: Option<Template>) -> http::Response
    where
        R: ResourceRequest<T>,
        R::Service: NewService<Response = T, Error = Error>;

    fn error<R>(self, error: Error, template: Option<Template>) -> http::Response
    where
        R: ResourceRequest<T>,
        R::Service: NewService<Response = T, Error = Error>;
}

pub trait PresentCollection<T: Resource>: Clone + Send + 'static {
    fn append(&mut self, resource: T, template: Option<Template>);
    fn error(&mut self, error: Error, template: Option<Template>);
    fn finish(self) -> http::Response;

    fn start<R>(&mut self)
    where
        R: CollectionRequest<T>,
        R::Service: NewStreamService<Response = T, Error = Error> { }
}

#[derive(Copy, Clone)]
pub struct Template<'a> {
    pub content_type: &'a str,
    pub template: &'a str,
}
