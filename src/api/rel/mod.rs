use api::{Resource, Entity, Result};

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
    fn has_one(entity: &Entity<Self>) -> Result<Option<RelationId<T>>>;
}

pub trait HasMany<T: Relation>: Resource {
    fn has_many(entity: &Entity<Self>) -> Result<Vec<RelationId<T>>>;
}

pub trait LinkOne<T: Relation>: HasOne<T> {
    fn link_one(entity: &Entity<Self>, rel_id: &RelationId<T>) -> Result<()>;
}

pub trait AppendLinks<T: Relation>: HasMany<T> {
    fn append_links(entity: &Entity<Self>, rel_ids: &[RelationId<T>]) -> Result<()>;
}

pub trait ReplaceLinks<T: Relation>: HasMany<T> {
    fn replace_links(entity: &Entity<Self>, rel_ids: &[RelationId<T>]) -> Result<()>;
}

pub trait UnlinkOne<T: Relation>: HasOne<T> {
    fn unlink_one(entity: &Entity<Self>) -> Result<()>;
}

pub trait RemoveLinks<T: Relation>: HasMany<T> {
    fn unlink_many(entity: &Entity<Self>, rel_ids: &[RelationId<T>]) -> Result<()>;
}

pub trait ClearLinks<T: Relation>: HasMany<T> {
    fn unlink_all(entity: &Entity<Self>) -> Result<()>;
}
