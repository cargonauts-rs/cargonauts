use api::{Entity, Error};
use api::raw::{RawUpdate, RawPatch, ResourceResponse};
use api::rel::{Relation, HasOne};
use Deserialize;
use futures::Future;
use IntoFuture;

pub trait PatchOne<I, T: Relation>: HasOne<T> where T::Resource: RawUpdate {
    type Patch: Deserialize;
    type PatchOneFut: IntoFuture<Item = ResourceResponse<I, T::Resource>, Error = Error>;
    fn patch_one(entity: &Entity<Self>, patch: Self::Patch, rels: <T::Resource as RawUpdate>::Relationships) -> Self::PatchOneFut;
}

impl<I, T, Rel> PatchOne<I, Rel> for T
where T:                HasOne<Rel>,
      Rel:              Relation,
      Rel::Resource:    RawPatch<I> {
    type Patch = <<Rel as Relation>::Resource as RawPatch<I>>::Patch;
    type PatchOneFut = Result<ResourceResponse<I, Rel::Resource>, Error>;
    fn patch_one(entity: &Entity<Self>, patch: Self::Patch, rels: <Rel::Resource as RawUpdate>::Relationships) -> Self::PatchOneFut {
        if let Some(id) = <T as HasOne<Rel>>::has_one(entity).into_future().wait()? {
            <Rel::Resource as RawPatch<I>>::patch(id, patch, rels).into_future().wait()
        } else { Err(Error::NotFound) }
    }
}
