mod identifier;
mod include;
mod relationship;

pub use api::get::{RawGet, GetResponse};
pub use api::index::{RawIndex, IndexResponse};
pub use api::patch::{RawPatch, RawPatchAsync, PatchResponse};
pub use api::post::{RawPost, RawPostAsync, PostResponse};

pub use self::identifier::Identifier;
pub use self::include::Include;
pub use self::relationship::{Relationship, RelationshipLinkage, FetchRelationships, UpdateRelationships, NoRelationships};

use api::Resource;

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
