use api::raw::{RawResource, Include};
use api::{Resource, Entity, Error};
use api::rel::{ToOne, ToMany, UpdateLink, ReplaceLinks, HasOne, HasMany, RelationId};
use router::IncludeQuery;
use IntoFuture;
use futures::Future;

pub trait _FetchRels<I>: RawResource {
    fn rels(entity: Entity<Self>, includes: &[IncludeQuery]) -> Result<(Self::FetchRels, Vec<Include<I>>), Error>;
}

pub trait _UpdateRels: RawResource {
    fn update_rels(entity: Entity<Self>, rels: Self::UpdateRels) -> Result<Self::FetchRels, Error>;
}

pub trait _MaybeUpdateLink<T: ToOne>: Resource {
    type _MaybeUpdateLinkFut: Future<Item = (), Error = Error>;
    fn update_link(entity: Entity<Self>, rel_id: Option<&RelationId<T>>) -> Self::_MaybeUpdateLinkFut;
}

impl<T, Rel> _MaybeUpdateLink<Rel> for T where T: HasOne<Rel>, Rel: ToOne {
    type _MaybeUpdateLinkFut = Box<Future<Item = (), Error = Error>>;
    default fn update_link(_: Entity<Self>, _: Option<&RelationId<Rel>>) -> Self::_MaybeUpdateLinkFut {
        Box::new(Err(Error::BadRequest).into_future())
    }
}

impl<T, Rel> _MaybeUpdateLink<Rel> for T where T: UpdateLink<Rel>, Rel: ToOne {
    fn update_link(entity: Entity<Self>, rel_id: Option<&RelationId<Rel>>) -> Self::_MaybeUpdateLinkFut {
        Box::new(<T as UpdateLink<Rel>>::update_link(entity, rel_id).into_future())
    }
}

pub trait _MaybeReplaceLinks<T: ToMany>: Resource {
    type _MaybeReplaceLinksFut: Future<Item = (), Error = Error>;
    fn replace_links(entity: Entity<Self>, rel_ids: &[RelationId<T>]) -> Self::_MaybeReplaceLinksFut;
}

impl<T, Rel> _MaybeReplaceLinks<Rel> for T where T: HasMany<Rel>, Rel: ToMany {
    type _MaybeReplaceLinksFut = Box<Future<Item = (), Error = Error>>;
    default fn replace_links(_: Entity<Self>, _: &[RelationId<Rel>]) -> Self::_MaybeReplaceLinksFut {
        Box::new(Err(Error::BadRequest).into_future())
    }
}

impl<T, Rel> _MaybeReplaceLinks<Rel> for T where T: ReplaceLinks<Rel>, Rel: ToMany {
    fn replace_links(entity: Entity<Self>, rel_ids: &[RelationId<Rel>]) -> Self::_MaybeReplaceLinksFut {
        Box::new(<T as ReplaceLinks<Rel>>::replace_links(entity, rel_ids).into_future())
    }
}
