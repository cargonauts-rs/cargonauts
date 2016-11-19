use api::raw::{RawFetch, RawUpdate, Include};
use api::{Resource, Entity, Error};
use api::rel::{ToOne, ToMany, LinkOne, UnlinkOne, ReplaceLinks, HasOne, HasMany, RelationId};
use router::IncludeQuery;
use IntoFuture;
use futures::Future;

pub trait _FetchRels<I>: RawFetch {
    fn rels<>(entity: &Entity<Self>, includes: &[IncludeQuery]) -> Result<(Self::Relationships, Vec<Include<I>>), Error>;
}

pub trait _UpdateRels: RawUpdate {
    fn update_rels(entity: &Entity<Self>, rels: <Self as RawUpdate>::Relationships)
        -> Result<<Self as RawFetch>::Relationships, Error>;
}

pub trait _MaybeLinkOne<T: ToOne>: Resource {
    type _MaybeLinkOneFut: IntoFuture<Item = (), Error = Error>;
    fn link_one(entity: &Entity<Self>, rel_id: &RelationId<T>) -> Self::_MaybeLinkOneFut;
}

impl<T, Rel> _MaybeLinkOne<Rel> for T where T: HasOne<Rel>, Rel: ToOne {
    type _MaybeLinkOneFut = Result<(), Error>;
    default fn link_one(_: &Entity<Self>, _: &RelationId<Rel>) -> Self::_MaybeLinkOneFut {
        Err(Error::BadRequest)
    }
}

impl<T, Rel> _MaybeLinkOne<Rel> for T where T: LinkOne<Rel>, Rel: ToOne {
    fn link_one(entity: &Entity<Self>, rel_id: &RelationId<Rel>) -> Self::_MaybeLinkOneFut {
        <T as LinkOne<Rel>>::link_one(entity, rel_id).into_future().wait()
    }
}

pub trait _MaybeUnlinkOne<T: ToOne>: Resource {
    type _MaybeUnlinkOneFut: IntoFuture<Item = (), Error = Error>;
    fn unlink_one(entity: &Entity<Self>) -> Self::_MaybeUnlinkOneFut;
}

impl<T, Rel> _MaybeUnlinkOne<Rel> for T where T: HasOne<Rel>, Rel: ToOne {
    type _MaybeUnlinkOneFut = Result<(), Error>;
    default fn unlink_one(_: &Entity<Self>) -> Self::_MaybeUnlinkOneFut {
        Err(Error::BadRequest)
    }
}

impl<T, Rel> _MaybeUnlinkOne<Rel> for T where T: UnlinkOne<Rel>, Rel: ToOne {
    fn unlink_one(entity: &Entity<Self>) -> Self::_MaybeUnlinkOneFut {
        <T as UnlinkOne<Rel>>::unlink_one(entity).into_future().wait()
    }
}

pub trait _MaybeReplaceLinks<T: ToMany>: Resource {
    type _MaybeReplaceLinksFut: IntoFuture<Item = (), Error = Error>;
    fn replace_links(entity: &Entity<Self>, rel_ids: &[RelationId<T>]) -> Self::_MaybeReplaceLinksFut;
}

impl<T, Rel> _MaybeReplaceLinks<Rel> for T where T: HasMany<Rel>, Rel: ToMany {
    type _MaybeReplaceLinksFut = Result<(), Error>;
    default fn replace_links(_: &Entity<Self>, _: &[RelationId<Rel>]) -> Self::_MaybeReplaceLinksFut {
        Err(Error::BadRequest)
    }
}

impl<T, Rel> _MaybeReplaceLinks<Rel> for T where T: ReplaceLinks<Rel>, Rel: ToMany {
    fn replace_links(entity: &Entity<Self>, rel_ids: &[RelationId<Rel>]) -> Self::_MaybeReplaceLinksFut {
        <T as ReplaceLinks<Rel>>::replace_links(entity, rel_ids).into_future().wait()
    }
}
