use api::Result;
use api::raw::{RawFetch, RawGet, RawUpdate};

pub trait AsyncJob<T>: RawGet + RawFetch<Relationships = ()> where T: RawUpdate {
    fn cache_rels(&mut self, rels: <T as RawUpdate>::Relationships) -> Result<()>;
}
