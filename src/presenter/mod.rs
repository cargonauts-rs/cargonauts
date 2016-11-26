use api::raw::{RawResource, ResourceResponse, CollectionResponse, RelResponse};
use api::Error;
use api::async::AsyncAction;
use api::async::raw::JobResponse;
use router::{self, Response, Router};
use Future;
use IntoFuture;

mod jsonapi;

pub use self::jsonapi::JsonApi;

pub trait Presenter<T: RawResource, R: Router>: Sized + 'static {
    type Include: 'static;
    fn prepare(field_set: Option<Vec<String>>, linker: R::LinkMaker) -> Self;
    fn present_resource(self, response: ResourceResponse<Self::Include, T>) -> R::Response;
    fn present_collection(self, response: CollectionResponse<Self::Include, T>) -> R::Response;
    fn present_rel(self, rel: RelResponse<Self::Include>) -> R::Response;
    fn present_err(self, error: Error) -> R::Response;
    fn present_no_content(self) -> R::Response;

    fn try_present<F, X>(self, presentable: F) -> Box<Future<Item = R::Response, Error = ()>>
    where
        F: IntoFuture<Item = X, Error = Error>,
        F::Future: 'static,
        X: Presentable<Self, T, R>,
    {
        Box::new(presentable.into_future().then(|result| match result {
            Ok(response)    => Ok(response.present(self)),
            Err(error)      => Ok(self.present_err(error)),
        }))
    }
}

pub trait ConvertInclude<T> {
    fn convert(attributes: T) -> Self;
}

pub trait Presentable<P: Presenter<T, R>, T: RawResource, R: Router> {
    fn present(self, presenter: P) -> R::Response;
}

impl<P, R> Presentable<P, (), R> for ()
where
    P: Presenter<(), R>,
    R: Router,
{
    fn present(self, presenter: P) -> R::Response {
        presenter.present_no_content()
    }
}

impl<P, T, R> Presentable<P, T, R> for ResourceResponse<P::Include, T>
where
    P: Presenter<T, R>,
    T: RawResource,
    R: Router,
{
    fn present(self, presenter: P) -> R::Response {
        presenter.present_resource(self)
    }
}

impl<P, T, R> Presentable<P, T, R> for CollectionResponse<P::Include, T>
where
    P: Presenter<T, R>,
    T: RawResource,
    R: Router,
{
    fn present(self, presenter: P) -> R::Response {
        presenter.present_collection(self)
    }
} 

impl<P, T, R> Presentable<P, T::Job, R> for JobResponse<T>
where
    P: Presenter<T::Job, R>,
    T: AsyncAction,
    R: Router,
{
    fn present(self, presenter: P) -> R::Response {
        let mut response = presenter.present_resource(ResourceResponse {
            resource: self.resource,
            includes: vec![],
        });
        response.set_status(router::Status::Accepted);
        //TODO set location header
        response
    }
}

impl<P, T, R> Presentable<P, T, R> for RelResponse<P::Include>
where
    P: Presenter<T, R>,
    T: RawResource,
    R: Router,
{
    fn present(self, presenter: P) -> R::Response {
        presenter.present_rel(self)
    }
}
