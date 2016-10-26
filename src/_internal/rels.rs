use api::raw::{RawFetch, RawUpdate, Include};
use api::{Entity, Error};

pub trait _FetchRels: RawFetch {
    fn rels(entity: &Entity<Self>, includes: &[String]) -> Result<(Self::Relationships, Vec<Include>), Error>;
}

pub trait _UpdateRels: RawUpdate {
    fn update_rels(entity: &Entity<Self>, rels: <Self as RawUpdate>::Relationships)
        -> Result<<Self as RawFetch>::Relationships, Error>;
}
