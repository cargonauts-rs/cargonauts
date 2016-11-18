use api::Error;
use api::raw::{RawFetch, RawUpdate, NoRelationships};

mod patch;
mod post;

pub use self::patch::PatchAsync;
pub use self::post::PostAsync;

pub mod raw {
    pub use api::async::patch::{RawPatchAsync, Asynchronous};
    pub use api::async::post::RawPostAsync;

    use api::async::AsyncAction;
    use api::raw::ResourceObject;

    pub struct JobResponse<T: AsyncAction> {
        pub resource: ResourceObject<T::Job>,
    }
}

pub trait AsyncJob<T>: RawFetch<Relationships = NoRelationships> where T: RawUpdate {
    fn cache_rels(&mut self, rels: <T as RawUpdate>::Relationships) -> Result<(), Error>;
}

pub trait AsyncAction: RawUpdate {
    type Job: AsyncJob<Self>;
}
