pub mod middleware;

use mainsail::{ResourceEndpoint, Error};
use method::Method;
use http;

pub trait Present<T: ResourceEndpoint>: Default {
    type ResourcePresenter: PresentResource<T>;
    type CollectionPresenter: PresentCollection<T>;

    fn for_resource(&self) -> Self::ResourcePresenter;
    fn for_collection(&self) -> Self::CollectionPresenter;
}

pub trait PresentResource<T: ResourceEndpoint>: Clone + Send + 'static {
    fn resource<M: ?Sized + Method<Response = T>>(self, resource: T, template: Option<Template>)
        -> http::Response;

    fn error<M: ?Sized + Method<Response = T>>(self, error: Error, template: Option<Template>)
        -> http::Response;
}

pub trait PresentCollection<T: ResourceEndpoint>: Clone + Send + 'static {
    fn append(&mut self, resource: T, template: Option<Template>);
    fn error(&mut self, error: Error, template: Option<Template>);
    fn finish(self) -> http::Response;

    fn start<M: ?Sized + Method<Response = T>>(&mut self) { }
}

#[derive(Copy, Clone)]
pub struct Template<'a> {
    pub content_type: &'a str,
    pub template: &'a str,
}
