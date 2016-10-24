use api::{Resource, Result};

mod fetch;
mod delete;
mod patch;
mod post;


pub mod raw {
    pub use api::rel::delete::{DeleteOne, Remove, Clear};
    pub use api::rel::fetch::{FetchOne, FetchMany};
    pub use api::rel::patch::PatchOne;
    pub use api::rel::post::{PostOne, Append};
}

pub trait Relation {
    type Resource: Resource;
    fn to_one() -> &'static str;
    fn to_many() -> &'static str;
}

impl<T: Resource> Relation for T {
    type Resource = T;
    fn to_one() -> &'static str {
        T::resource()
    }
    fn to_many() -> &'static str {
        T::resource_plural()
    }
}

pub type RelationId<T> = <<T as Relation>::Resource as Resource>::Id;

pub trait HasOne<T: Relation>: Resource {
    fn has_one(id: &Self::Id) -> Result<Option<RelationId<T>>>;
}

pub trait HasMany<T: Relation>: Resource {
    fn has_many(id: &Self::Id) -> Result<Vec<RelationId<T>>>;
}

pub trait LinkOne<T: Relation>: HasOne<T> {
    fn link_one(id: &Self::Id, rel_id: &RelationId<T>) -> Result<()>;
}

pub trait AppendLinks<T: Relation>: HasMany<T> {
    fn append_links(id: &Self::Id, rel_ids: &[RelationId<T>]) -> Result<()>;
}

pub trait ReplaceLinks<T: Relation>: HasMany<T> {
    fn replace_links(id: &Self::Id, rel_ids: &[RelationId<T>]) -> Result<()>;
}

pub trait UnlinkOne<T: Relation>: HasOne<T> {
    fn unlink_one(id: &Self::Id) -> Result<()>;
}

pub trait RemoveLinks<T: Relation>: HasMany<T> {
    fn unlink_many(id: &Self::Id, rel_ids: &[RelationId<T>]) -> Result<()>;
}

pub trait ClearLinks<T: Relation>: HasMany<T> {
    fn unlink_all(id: &Self::Id) -> Result<()>;
}
