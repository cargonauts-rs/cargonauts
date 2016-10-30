use api::raw::{RawFetch, RawUpdate, Include};
use api::{Resource, Entity, Result, Error};
use api::rel::{Relation, LinkOne, UnlinkOne, ReplaceLinks, HasOne, HasMany, RelationId};
use router::IncludeQuery;
use presenter::Presenter;

pub trait _FetchRels: RawFetch {
    fn rels<P: Presenter>(entity: &Entity<Self>, includes: &[IncludeQuery]) -> Result<(Self::Relationships, Vec<Include<P>>)>;
}

pub trait _UpdateRels: RawUpdate {
    fn update_rels(entity: &Entity<Self>, rels: <Self as RawUpdate>::Relationships)
        -> Result<<Self as RawFetch>::Relationships>;
}

pub trait _MaybeLinkOne<T: Relation>: Resource {
    fn link_one(entity: &Entity<Self>, rel_id: &RelationId<T>) -> Result<()>;
}

impl<T, Rel> _MaybeLinkOne<Rel> for T where T: HasOne<Rel>, Rel: Relation {
    default fn link_one(_: &Entity<Self>, _: &RelationId<Rel>) -> Result<()> {
        Err(Error::BadRequest)
    }
}

impl<T, Rel> _MaybeLinkOne<Rel> for T where T: LinkOne<Rel>, Rel: Relation {
    fn link_one(entity: &Entity<Self>, rel_id: &RelationId<Rel>) -> Result<()> {
        <T as LinkOne<Rel>>::link_one(entity, rel_id)
    }
}

pub trait _MaybeUnlinkOne<T: Relation>: Resource {
    fn unlink_one(entity: &Entity<Self>) -> Result<()>;
}

impl<T, Rel> _MaybeUnlinkOne<Rel> for T where T: HasOne<Rel>, Rel: Relation {
    default fn unlink_one(_: &Entity<Self>) -> Result<()> {
        Err(Error::BadRequest)
    }
}

impl<T, Rel> _MaybeUnlinkOne<Rel> for T where T: UnlinkOne<Rel>, Rel: Relation {
    fn unlink_one(entity: &Entity<Self>) -> Result<()> {
        <T as UnlinkOne<Rel>>::unlink_one(entity)
    }
}

pub trait _MaybeReplaceLinks<T: Relation>: Resource {
    fn replace_links(entity: &Entity<Self>, rel_ids: &[RelationId<T>]) -> Result<()>;
}

impl<T, Rel> _MaybeReplaceLinks<Rel> for T where T: HasMany<Rel>, Rel: Relation {
    default fn replace_links(_: &Entity<Self>, _: &[RelationId<Rel>]) -> Result<()> {
        Err(Error::BadRequest)
    }
}

impl<T, Rel> _MaybeReplaceLinks<Rel> for T where T: ReplaceLinks<Rel>, Rel: Relation {
    fn replace_links(entity: &Entity<Self>, rel_ids: &[RelationId<Rel>]) -> Result<()> {
        <T as ReplaceLinks<Rel>>::replace_links(entity, rel_ids)
    }
}
