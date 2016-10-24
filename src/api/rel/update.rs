use api::{Resource, Error};
use api::rel::{Relation, HasOne, HasMany};

pub trait LinkMany<T: Relation>: HasMany<T> {
    fn link(id: &Self::Id, rel_ids: &[<T::Resource as Resource>::Id]) -> Result<(), Error>;
}

pub trait AppendMany<T: Relation>: HasMany<T> {
    fn append(id: &Self::Id, rel_ids: &[<T::Resource as Resource>::Id]) -> Result<(), Error>;
}
