use api::Error;
use api::raw::{RawFetch, RawUpdate, NoRelationships};

pub trait AsyncJob<T>: RawFetch<Relationships = NoRelationships> where T: RawUpdate {
    fn cache_rels(&mut self, rels: <T as RawUpdate>::Relationships) -> Result<(), Error>;
}

pub trait AsyncAction: RawUpdate {
    type Job: AsyncJob<Self>;
}
