use api::Result;
use api::raw::{RawFetch, RawGet, RawUpdate, NoRelationships};

pub trait AsyncJob<T>: RawGet + RawFetch<Relationships = NoRelationships> where T: RawUpdate {
    fn cache_rels(&mut self, rels: <T as RawUpdate>::Relationships) -> Result<()>;
}
