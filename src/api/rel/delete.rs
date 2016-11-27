use api::{Error, Delete, Entity, Remove};
use api::rel::{ToOne, ToMany, UnlinkOne, RemoveLinks, RelationId};
use futures::Future;
use IntoFuture;

pub trait DeleteOne<T: ToOne>: UnlinkOne<T> {
    type DeleteOneFut: Future<Item = (), Error = Error> + 'static;
    fn delete_one(entity: &Entity<Self>) -> Self::DeleteOneFut;
}

impl<T, Rel> DeleteOne<Rel> for T
where T:             UnlinkOne<Rel>,
      Rel:           ToOne,
      Rel::Resource: Delete {
    type DeleteOneFut = Box<Future<Item = (), Error = Error>>;
    fn delete_one(entity: &Entity<Self>) -> Self::DeleteOneFut {
        Box::new(<T as UnlinkOne<Rel>>::unlink_one(entity).into_future().and_then(move |id| {
            if let Some(rel_id) = id {
                Box::new(<Rel::Resource as Delete>::delete(&rel_id).into_future()) as Box<Future<Item = (), Error = Error>>
            } else {
                Box::new(Ok(()).into_future())
            }
        }))
    }
}

pub trait RemoveMany<T: ToMany>: RemoveLinks<T> {
    type RemoveManyFut: IntoFuture<Item = (), Error = Error> + 'static;
    fn remove_many(entity: &Entity<Self>, rel_ids: &[RelationId<T>]) -> Self::RemoveManyFut;
}

impl<T, Rel> RemoveMany<Rel> for T
where T:             RemoveLinks<Rel>,
      Rel:           ToMany,
      Rel::Resource: Remove {
    type RemoveManyFut = Box<Future<Item = (), Error = Error>>;
    fn remove_many(entity: &Entity<Self>, rel_ids: &[RelationId<Rel::Resource>]) -> Self::RemoveManyFut {
        Box::new(<T as RemoveLinks<Rel>>::remove_links(entity, rel_ids).into_future().and_then(|rel_ids| {
            <Rel::Resource as Remove>::remove(&rel_ids)
        }))
    }
}
