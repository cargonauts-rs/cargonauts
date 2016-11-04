use api::raw::{ResourceObject, Include, RawFetch, Relationship};
use api::Error;
use router::{Response, Linker};

mod jsonapi;

pub use self::jsonapi::JsonApi;

pub trait Presenter<T: RawFetch>: Sized {
    type Response: Response;
    type Linker: Linker;
    type Include;
    fn prepare(field_set: Option<Vec<String>>, linker: Self::Linker) -> Self;
    fn present_resource(self, resource: ResourceObject<T>, includes: Vec<Include<Self::Include>>) -> Self::Response;
    fn present_collection(self, resources: Vec<ResourceObject<T>>, includes: Vec<Include<Self::Include>>) -> Self::Response;
    fn present_rel(self, resource: &str, id: &str, name: &str, rel: Relationship, includes: Vec<Include<Self::Include>>) -> Self::Response;
    fn present_nil(self, self_link: &str) -> Self::Response;
    fn present_err(self, error: Error) -> Self::Response;
}

pub trait ConvertInclude<T> {
    fn convert(attributes: T) -> Self;
}
