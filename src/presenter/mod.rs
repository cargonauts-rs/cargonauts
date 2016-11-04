use api::raw::{ResourceObject, Include, RawFetch, Relationship};
use api::Error;
use router::{Response, Linker};

mod jsonapi;

pub use self::jsonapi::JsonApi;

pub trait Presenter<R: Response, L: Linker>: Sized {
    fn prepare(field_set: Option<Vec<String>>, linker: L) -> Self;
    fn present_resource<T>(self, resource: ResourceObject<T>, includes: Vec<Include<R::Serializer>>) -> R
        where T: RawFetch;
    fn present_collection<T>(self, resources: Vec<ResourceObject<T>>, includes: Vec<Include<R::Serializer>>) -> R
        where T: RawFetch;
    fn present_rel(self, resource: &str, id: &str, name: &str, rel: Relationship, includes: Vec<Include<R::Serializer>>) -> R;
    fn present_nil(self, self_link: &str) -> R;
    fn present_err(self, error: Error) -> R;
}
