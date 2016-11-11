use api::{Entity, Error};
use api::raw::{RawUpdate, RawPatch, PatchResponse};
use api::rel::{Relation, HasOne};
use Deserialize;
use futures::Future;
use IntoFuture;

pub trait PatchOne<T: Relation>: HasOne<T> where T::Resource: RawUpdate {
    type Patch: Deserialize;
    type PatchOneFut: IntoFuture<Item = PatchResponse<T::Resource>, Error = Error>;
    fn patch_one(entity: &Entity<Self>, patch: Self::Patch, rels: <T::Resource as RawUpdate>::Relationships) -> Self::PatchOneFut;
}

impl<T, Rel> PatchOne<Rel> for T
where T:                HasOne<Rel>,
      Rel:              Relation,
      Rel::Resource:    RawPatch {
    type Patch = <<Rel as Relation>::Resource as RawPatch>::Patch;
    type PatchOneFut = Result<PatchResponse<Rel::Resource>, Error>;
    fn patch_one(entity: &Entity<Self>, patch: Self::Patch, rels: <Rel::Resource as RawUpdate>::Relationships) -> Self::PatchOneFut {
        if let Some(id) = <T as HasOne<Rel>>::has_one(entity).into_future().wait()? {
            <Rel::Resource as RawPatch>::patch(id, patch, rels).into_future().wait()
        } else { Err(Error::NotFound) }
    }
}
