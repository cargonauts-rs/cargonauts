mod identifier;
mod include;
mod relationship;

pub use api::alias::RawGetAliased;
pub use api::get::RawGet;
pub use api::index::RawIndex;
pub use api::patch::{RawPatch, RawPatchAsync};
pub use api::post::{RawPost, RawPostAsync};

pub use self::identifier::Identifier;
pub use self::include::Include;
pub use self::relationship::{Relationship, RelationshipLinkage, FetchRelationships, UpdateRelationships, NoRelationships};

use api::{Resource, AsyncAction};

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

impl RawFetch for () {
    type Relationships = NoRelationships;
}

pub struct ResourceResponse<I, T: RawFetch> {
    pub resource: ResourceObject<T>,
    pub includes: Vec<Include<I>>,
}

pub struct CollectionResponse<I, T: RawFetch> {
    pub resources: Vec<ResourceObject<T>>,
    pub includes: Vec<Include<I>>,
}

pub struct JobResponse<T: AsyncAction> {
    pub resource: ResourceObject<T::Job>,
}
