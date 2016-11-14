use api::{Error, Delete, Entity, Remove};
use api::rel::{Relation, HasOne, HasMany, UnlinkOne, RemoveLinks, ClearLinks, RelationId};
use futures::Future;
use IntoFuture;

pub trait DeleteOne<T: Relation>: HasOne<T> + UnlinkOne<T> {
    type DeleteOneFut: IntoFuture<Item = (), Error = Error>;
    fn delete_one(entity: &Entity<Self>) -> Self::DeleteOneFut;
}

impl<T, Rel> DeleteOne<Rel> for T
where T:             HasOne<Rel> + UnlinkOne<Rel>,
      Rel:           Relation,
      Rel::Resource: Delete {
    type DeleteOneFut = Result<(), Error>;
    fn delete_one(entity: &Entity<Self>) -> Self::DeleteOneFut {
        if let Some(rel_id) = <T as HasOne<Rel>>::has_one(entity).into_future().wait()? {
            <Rel::Resource as Delete>::delete(&rel_id).into_future().wait()?;
            <T as UnlinkOne<Rel>>::unlink_one(entity).into_future().wait()
        } else { Ok(()) }
    }
}

pub trait RemoveMany<T: Relation>: HasMany<T> + RemoveLinks<T> {
    type RemoveManyFut: IntoFuture<Item = (), Error = Error>;
    fn remove_many(entity: &Entity<Self>, rel_ids: &[RelationId<T>]) -> Self::RemoveManyFut;
}

impl<T, Rel> RemoveMany<Rel> for T
where T:             HasMany<Rel> + RemoveLinks<Rel>,
      Rel:           Relation,
      Rel::Resource: Remove {
    type RemoveManyFut = Result<(), Error>;
    fn remove_many(entity: &Entity<Self>, rel_ids: &[RelationId<Rel::Resource>]) -> Self::RemoveManyFut {
        let rel_ids: Vec<_> = <T as HasMany<Rel>>::has_many(entity).into_future().wait()?.into_iter().filter(|id| rel_ids.contains(id)).collect();
        <Rel::Resource as Remove>::remove(&rel_ids).into_future().wait()?;
        <T as RemoveLinks<Rel>>::remove_links(entity, &rel_ids).into_future().wait()
    }
}

pub trait ClearMany<T: Relation>: HasMany<T> + ClearLinks<T> {
    type ClearManyFut: IntoFuture<Item = (), Error = Error>;
    fn clear_many(entity: &Entity<Self>) -> Self::ClearManyFut;
}

impl<T, Rel> ClearMany<Rel> for T
where T:             HasMany<Rel> + ClearLinks<Rel>,
      Rel:           Relation,
      Rel::Resource: Remove {
    type ClearManyFut = Result<(), Error>;
    fn clear_many(entity: &Entity<Self>) -> Self::ClearManyFut {
        let rel_ids: Vec<_> = <T as HasMany<Rel>>::has_many(entity).into_future().wait()?;
        <Rel::Resource as Remove>::remove(&rel_ids).into_future().wait()?;
        <T as ClearLinks<Rel>>::clear_links(entity).into_future().wait()
    }
}
