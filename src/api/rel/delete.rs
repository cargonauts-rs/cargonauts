use api::{Error, Delete, Entity};
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

pub trait Remove<T: Relation>: HasMany<T> + RemoveLinks<T> {
    type RemoveFut: IntoFuture<Item = (), Error = Error>;
    fn remove_many(entity: &Entity<Self>, rel_ids: &[RelationId<T>]) -> Self::RemoveFut;
}

impl<T, Rel> Remove<Rel> for T
where T:             HasMany<Rel> + RemoveLinks<Rel>,
      Rel:           Relation,
      Rel::Resource: Delete {
    type RemoveFut = Result<(), Error>;
    fn remove_many(entity: &Entity<Self>, rel_ids: &[RelationId<Rel::Resource>]) -> Self::RemoveFut {
        let rel_ids: Vec<_> = <T as HasMany<Rel>>::has_many(entity).into_future().wait()?.into_iter().filter(|id| rel_ids.contains(id)).collect();
        for id in &rel_ids {
            <Rel::Resource as Delete>::delete(id).into_future().wait()?;
        }
        <T as RemoveLinks<Rel>>::remove_links(entity, &rel_ids).into_future().wait()
    }
}

pub trait Clear<T: Relation>: HasMany<T> + ClearLinks<T> {
    type ClearFut: IntoFuture<Item = (), Error = Error>;
    fn clear_many(entity: &Entity<Self>) -> Self::ClearFut;
}

impl<T, Rel> Clear<Rel> for T
where T:             HasMany<Rel> + ClearLinks<Rel>,
      Rel:           Relation,
      Rel::Resource: Delete {
    type ClearFut = Result<(), Error>;
    fn clear_many(entity: &Entity<Self>) -> Self::ClearFut {
        for id in <T as HasMany<Rel>>::has_many(entity).into_future().wait()? {
            <Rel::Resource as Delete>::delete(&id).into_future().wait()?;
        }
        <T as ClearLinks<Rel>>::clear_links(entity).into_future().wait()
    }
}
