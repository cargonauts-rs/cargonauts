use api::{Error, Delete, Entity};
use api::rel::{Relation, HasOne, HasMany, UnlinkOne, RemoveLinks, ClearLinks, RelationId};

pub trait DeleteOne<T: Relation>: HasOne<T> + UnlinkOne<T> {
    fn delete_one(entity: &Entity<Self>) -> Result<(), Error>;
}

impl<T, Rel> DeleteOne<Rel> for T
where T:             HasOne<Rel> + UnlinkOne<Rel>,
      Rel:           Relation,
      Rel::Resource: Delete {
    fn delete_one(entity: &Entity<Self>) -> Result<(), Error> {
        if let Some(rel_id) = try!(<T as HasOne<Rel>>::has_one(entity)) {
            try!(<Rel::Resource as Delete>::delete(&rel_id));
            <T as UnlinkOne<Rel>>::unlink_one(entity)
        } else { Ok(()) }
    }
}

pub trait Remove<T: Relation>: HasMany<T> + RemoveLinks<T> {
    fn remove_many(entity: &Entity<Self>, rel_ids: &[RelationId<T>]) -> Result<(), Error>;
}

impl<T, Rel> Remove<Rel> for T
where T:             HasMany<Rel> + RemoveLinks<Rel>,
      Rel:           Relation,
      Rel::Resource: Delete {
    fn remove_many(entity: &Entity<Self>, rel_ids: &[RelationId<Rel::Resource>]) -> Result<(), Error> {
        let rel_ids: Vec<_> = try!(<T as HasMany<Rel>>::has_many(entity)).into_iter().filter(|id| rel_ids.contains(id)).collect();
        for id in &rel_ids {
            try!(<Rel::Resource as Delete>::delete(id));
        }
        <T as RemoveLinks<Rel>>::unlink_many(entity, &rel_ids)
    }
}

pub trait Clear<T: Relation>: HasMany<T> + ClearLinks<T> {
    fn clear_many(entity: &Entity<Self>) -> Result<(), Error>;
}

impl<T, Rel> Clear<Rel> for T
where T:             HasMany<Rel> + ClearLinks<Rel>,
      Rel:           Relation,
      Rel::Resource: Delete {
    fn clear_many(entity: &Entity<Self>) -> Result<(), Error> {
        for id in try!(<T as HasMany<Rel>>::has_many(entity)) {
            try!(<Rel::Resource as Delete>::delete(&id));
        }
        <T as ClearLinks<Rel>>::unlink_all(entity)
    }
}
