use api::{Resource, Entity, Error};
use IntoFuture;

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
    type HasOneFut: IntoFuture<Item = Option<RelationId<T>>, Error = Error>;
    fn has_one(entity: &Entity<Self>) -> Self::HasOneFut;
}

pub trait HasMany<T: Relation>: Resource {
    type HasManyFut: IntoFuture<Item = Vec<RelationId<T>>, Error = Error>;
    fn has_many(entity: &Entity<Self>) -> Self::HasManyFut;
}

pub trait LinkOne<T: Relation>: HasOne<T> {
    type LinkOneFut: IntoFuture<Item = (), Error = Error>;
    fn link_one(entity: &Entity<Self>, rel_id: &RelationId<T>) -> Self::LinkOneFut;
}

pub trait AppendLinks<T: Relation>: HasMany<T> {
    type AppendLinksFut: IntoFuture<Item = (), Error = Error>;
    fn append_links(entity: &Entity<Self>, rel_ids: &[RelationId<T>]) -> Self::AppendLinksFut;
}

pub trait ReplaceLinks<T: Relation>: HasMany<T> {
    type ReplaceLinksFut: IntoFuture<Item = (), Error = Error>;
    fn replace_links(entity: &Entity<Self>, rel_ids: &[RelationId<T>]) -> Self::ReplaceLinksFut;
}

pub trait UnlinkOne<T: Relation>: HasOne<T> {
    type UnlinkOneFut: IntoFuture<Item = (), Error = Error>;
    fn unlink_one(entity: &Entity<Self>) -> Self::UnlinkOneFut;
}

pub trait RemoveLinks<T: Relation>: HasMany<T> {
    type RemoveLinksFut: IntoFuture<Item = (), Error = Error>;
    fn remove_links(entity: &Entity<Self>, rel_ids: &[RelationId<T>]) -> Self::RemoveLinksFut;
}

pub trait ClearLinks<T: Relation>: HasMany<T> {
    type ClearLinksFut: IntoFuture<Item = (), Error = Error>;
    fn clear_links(entity: &Entity<Self>) -> Self::ClearLinksFut;
}
