use api::raw::{RawResource, Include};
use api::{Resource, Entity, Error};
use api::rel::{ToOne, ToMany, LinkOne, UnlinkOne, ReplaceLinks, HasOne, HasMany, RelationId};
use router::IncludeQuery;
use IntoFuture;
use futures::Future;

pub trait _FetchRels<I>: RawResource {
    fn rels(entity: &Entity<Self>, includes: &[IncludeQuery]) -> Result<(Self::FetchRels, Vec<Include<I>>), Error>;
}

pub trait _UpdateRels: RawResource {
    fn update_rels(entity: &Entity<Self>, rels: Self::UpdateRels) -> Result<Self::FetchRels, Error>;
}

pub trait _MaybeLinkOne<T: ToOne>: Resource {
    type _MaybeLinkOneFut: Future<Item = (), Error = Error>;
    fn link_one(entity: &Entity<Self>, rel_id: &RelationId<T>) -> Self::_MaybeLinkOneFut;
}

impl<T, Rel> _MaybeLinkOne<Rel> for T where T: HasOne<Rel>, Rel: ToOne {
    type _MaybeLinkOneFut = Box<Future<Item = (), Error = Error>>;
    default fn link_one(_: &Entity<Self>, _: &RelationId<Rel>) -> Self::_MaybeLinkOneFut {
        Box::new(Err(Error::BadRequest).into_future())
    }
}

impl<T, Rel> _MaybeLinkOne<Rel> for T where T: LinkOne<Rel>, Rel: ToOne {
    fn link_one(entity: &Entity<Self>, rel_id: &RelationId<Rel>) -> Self::_MaybeLinkOneFut {
        Box::new(<T as LinkOne<Rel>>::link_one(entity, rel_id).into_future())
    }
}

pub trait _MaybeUnlinkOne<T: ToOne>: Resource {
    type _MaybeUnlinkOneFut: Future<Item = (), Error = Error>;
    fn unlink_one(entity: &Entity<Self>) -> Self::_MaybeUnlinkOneFut;
}

impl<T, Rel> _MaybeUnlinkOne<Rel> for T where T: HasOne<Rel>, Rel: ToOne {
    type _MaybeUnlinkOneFut = Box<Future<Item = (), Error = Error>>;
    default fn unlink_one(_: &Entity<Self>) -> Self::_MaybeUnlinkOneFut {
        Box::new(Err(Error::BadRequest).into_future())
    }
}

impl<T, Rel> _MaybeUnlinkOne<Rel> for T where T: UnlinkOne<Rel>, Rel: ToOne {
    fn unlink_one(entity: &Entity<Self>) -> Self::_MaybeUnlinkOneFut {
        Box::new(<T as UnlinkOne<Rel>>::unlink_one(entity).into_future().map(|_| ()))
    }
}

pub trait _MaybeReplaceLinks<T: ToMany>: Resource {
    type _MaybeReplaceLinksFut: Future<Item = (), Error = Error>;
    fn replace_links(entity: &Entity<Self>, rel_ids: &[RelationId<T>]) -> Self::_MaybeReplaceLinksFut;
}

impl<T, Rel> _MaybeReplaceLinks<Rel> for T where T: HasMany<Rel>, Rel: ToMany {
    type _MaybeReplaceLinksFut = Box<Future<Item = (), Error = Error>>;
    default fn replace_links(_: &Entity<Self>, _: &[RelationId<Rel>]) -> Self::_MaybeReplaceLinksFut {
        Box::new(Err(Error::BadRequest).into_future())
    }
}

impl<T, Rel> _MaybeReplaceLinks<Rel> for T where T: ReplaceLinks<Rel>, Rel: ToMany {
    fn replace_links(entity: &Entity<Self>, rel_ids: &[RelationId<Rel>]) -> Self::_MaybeReplaceLinksFut {
        Box::new(<T as ReplaceLinks<Rel>>::replace_links(entity, rel_ids).into_future())
    }
}
