use api::raw::{ResourceObject, Include, RawFetch, Relationship};
use api::Error;
use router::Response;

mod jsonapi;

pub use self::jsonapi::JsonApi;

pub trait Presenter<R: Response>: Sized {
    fn prepare(field_set: Option<Vec<String>>) -> Self;
    fn present_resource<T>(self, self_link: &str, resource: ResourceObject<T>, includes: Vec<Include<R::Serializer>>) -> R
        where T: RawFetch;
    fn present_collection<T>(self, self_link: &str, resources: Vec<ResourceObject<T>>, includes: Vec<Include<R::Serializer>>) -> R
        where T: RawFetch;
    fn present_rel(self, self_link: &str, rel_link: &str, rel: Relationship, includes: Vec<Include<R::Serializer>>) -> R;
    fn present_nil(self, self_link: &str) -> R;
    fn present_err(self, error: Error) -> R;
}
