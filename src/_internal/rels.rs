use api::raw::{RawFetch, RawUpdate, Include};
use api::Error;

pub trait _FetchRels: RawFetch {
    fn rels(id: &Self::Id, includes: &[String]) -> Result<(Self::Relationships, Vec<Include>), Error>;
}

pub trait _UpdateRels: RawUpdate {
    fn update_rels(id: &Self::Id, rels: <Self as RawUpdate>::Relationships)
        -> Result<<Self as RawFetch>::Relationships, Error>;
}
