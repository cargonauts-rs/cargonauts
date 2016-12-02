use api::{Resource, Entity, Error};
use Future;
use IntoFuture;
use futures::stream::{self, Stream};

mod fetch;
mod delete;
mod patch;
mod post;

pub mod raw {
    pub use api::rel::delete::{DeleteOne, RemoveMany};
    pub use api::rel::fetch::{GetOne, IndexMany};
    pub use api::rel::patch::PatchOne;
    pub use api::rel::post::{PostMany, ReplaceMany};
}

pub trait Relation {
    type Resource: Resource;
}

pub trait ToOne: Relation {
    fn to_one() -> &'static str;
}

pub trait ToMany: Relation {
    fn to_many() -> &'static str;
}

impl<T: Resource> Relation for T {
    type Resource = T;
}

impl<T: Resource> ToOne for T {
    fn to_one() -> &'static str {
        T::resource()
    }
}

impl<T: Resource> ToMany for T {
    fn to_many() -> &'static str {
        T::resource_plural()
    }
}

pub type RelationId<T> = <<T as Relation>::Resource as Resource>::Id;

pub trait HasOne<T: ToOne>: Resource {
    type HasOneFut: IntoFuture<Item = Option<RelationId<T>>, Error = Error> + 'static;
    fn has_one(entity: Entity<Self>) -> Self::HasOneFut;
}

pub trait HasMany<T: ToMany>: Resource {
    type HasManyFut: IntoFuture<Item = Vec<RelationId<T>>, Error = Error> + 'static;
    fn has_many(entity: Entity<Self>) -> Self::HasManyFut;
}

pub trait UpdateLink<T: ToOne>: HasOne<T> {
    type UpdateLinkFut: IntoFuture<Item = (), Error = Error> + 'static;
    fn update_link(entity: Entity<Self>, rel_id: Option<&RelationId<T>>) -> Self::UpdateLinkFut;
}

pub trait PostLinks<T: ToMany>: HasMany<T> {
    type PostLinksFut: IntoFuture<Item = (), Error = Error> + 'static;
    fn post_link(entity: Entity<Self>, rel_id: RelationId<T>) -> Self::PostLinksFut;
    fn append_links(entity: Entity<Self>, rel_id: Vec<RelationId<T>>) -> Box<Future<Item = (), Error = Error>> {
        Box::new(stream::iter(rel_id.into_iter().map(Ok)).and_then(move |id| Self::post_link(entity.clone(), id)).for_each(|_| Ok(())))
    }
}

pub trait ReplaceLinks<T: ToMany>: HasMany<T> {
    type ReplaceLinksFut: IntoFuture<Item = (), Error = Error> + 'static;
    fn replace_links(entity: Entity<Self>, rel_ids: &[RelationId<T>]) -> Self::ReplaceLinksFut;
}

pub trait RemoveLinks<T: ToMany>: HasMany<T> {
    type RemoveLinksFut: IntoFuture<Item = Vec<RelationId<T>>, Error = Error> + 'static;
    fn remove_links(entity: Entity<Self>, rel_ids: &[RelationId<T>]) -> Self::RemoveLinksFut;
}
