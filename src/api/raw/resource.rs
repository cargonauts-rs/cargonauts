use api::Resource;
use api::raw::{FetchRelationships, UpdateRelationships};

pub trait RawFetch: Resource {
    type Relationships: for<'a> FetchRelationships<'a>;
}

pub trait RawUpdate: RawFetch {
    type Relationships: UpdateRelationships;
}

pub struct ResourceObject<T: RawFetch> {
    pub id: <T as Resource>::Id,
    pub attributes: T,
    pub relationships: <T as RawFetch>::Relationships,
}
