mod identifier;
mod include;
mod relationship;

pub use api::alias::{RawGetAliased, RawIndexAliased};

pub use api::append::RawAppend;
pub use api::get::RawGet;
pub use api::index::RawIndex;
pub use api::patch::{RawHasPatch, RawPatch, Synchronous};
pub use api::post::RawPost;
pub use api::replace::RawReplace;

pub use self::identifier::Identifier;
pub use self::include::Include;
pub use self::relationship::{Relationship, RelationshipLinkage, FetchRelationships, UpdateRelationships, NoRelationships, RelationshipError};

use api::Resource;

pub trait RawResource: Resource {
    type FetchRels: for<'a> FetchRelationships<'a>;
    type UpdateRels: UpdateRelationships;
}

pub struct ResourceObject<T: RawResource> {
    pub id: <T as Resource>::Id,
    pub attributes: T,
    pub relationships: T::FetchRels,
}

impl RawResource for () {
    type FetchRels = NoRelationships;
    type UpdateRels = NoRelationships;
}

pub struct RawReceived<T: RawResource, A> {
    pub attributes: A,
    pub relationships: T::UpdateRels,
}


pub struct ResourceResponse<I, T: RawResource> {
    pub resource: ResourceObject<T>,
    pub includes: Vec<Include<I>>,
}

pub struct CollectionResponse<I, T: RawResource> {
    pub resources: Vec<ResourceObject<T>>,
    pub includes: Vec<Include<I>>,
}

impl<I, T: RawResource> Default for CollectionResponse<I, T> {
    fn default() -> CollectionResponse<I, T> {
        CollectionResponse {
            resources: vec![],
            includes: vec![],
        }
    }
}

pub struct RelResponse<I> {
    pub resource: &'static str,
    pub related: &'static str,
    pub id: String,
    pub rel: Relationship,
    pub includes: Vec<Include<I>>,
}
