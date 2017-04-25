use {ResourceEndpoint, Error};
use environment::Environment;
use method::Method;
use request::Request;
use http;

pub trait Format<T: ResourceEndpoint, M: ?Sized + Method<T>> {
    type Receiver: Receive<T, M>;
    type Presenter: Present<T, M>;
}

pub trait Receive<T: ResourceEndpoint, M: ?Sized + Method<T>> {
    fn receive(req: http::Request, env: &mut Environment) -> Result<<M::Request as Request<T>>::BodyParts, Error>;
}

pub trait Present<T: ResourceEndpoint, M: ?Sized + Method<T>>: Send + 'static {
    type ResourcePresenter: PresentResource<T, M>;
    type CollectionPresenter: PresentCollection<T, M>;

    fn for_resource() -> Self::ResourcePresenter;
    fn for_collection() -> Self::CollectionPresenter;
}

pub trait PresentResource<T: ResourceEndpoint, M: ?Sized + Method<T>> {
    fn resource(self, resource: M::Response, template: Option<Template>) -> http::Response;
    fn error(self, error: Error, template: Option<Template>) -> http::Response;
}

pub trait PresentCollection<T: ResourceEndpoint, M: ?Sized + Method<T>>: Send + 'static {
    fn append(&mut self, resource: M::Response, template: Option<Template>);
    fn error(&mut self, error: Error, template: Option<Template>);
    fn finish(self) -> http::Response;
}

#[derive(Copy, Clone)]
pub struct Template;
