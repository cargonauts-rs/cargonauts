use api::Error;
use api::raw::{RawFetch, RawUpdate, NoRelationships};

mod patch;
mod post;

pub use self::patch::PatchAsync;
pub use self::post::PostAsync;

pub mod raw {
    pub use api::async::patch::RawPatchAsync;
    pub use api::async::post::RawPostAsync;
}

pub trait AsyncJob<T>: RawFetch<Relationships = NoRelationships> where T: RawUpdate {
    fn cache_rels(&mut self, rels: <T as RawUpdate>::Relationships) -> Result<(), Error>;
}

pub trait AsyncAction: RawUpdate {
    type Job: AsyncJob<Self>;
}
