use super::*;

pub trait HasOne<T: Resource>: Resource {
    fn has_one(id: &Self::Id) -> Option<T>;
}

pub trait HasMany<T: Resource>: Resource {
    fn has_many(id: &Self::Id) -> Vec<T>;
}

pub trait UpdateOne<T: Resource>: HasOne<T> {
    fn link(id: &Self::Id, rel_id: &T::Id) -> Result<(), LinkError>;
}

pub trait UpdateMany<T: Resource>: HasMany<T> {
    fn link(id: &Self::Id, rel_ids: &[T::Id]) -> Result<(), LinkError>;
}

pub trait DeleteOne<T: Resource>: HasOne<T> {
    fn unlink(id: &Self::Id) -> Result<(), DeleteError>;
}

pub trait DeleteMany<T: Resource>: HasMany<T> {
    fn unlink(id: &Self::Id, rel_ids: &[T::Id]) -> Result<(), DeleteError>;
}
