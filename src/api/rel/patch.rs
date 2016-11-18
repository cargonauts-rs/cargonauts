use api::{Entity, Error};
use api::raw::{RawReceived, RawHasPatch, RawPatch, ResourceResponse, Synchronous};
use api::rel::{Relation, HasOne};
use futures::Future;
use IntoFuture;

pub trait PatchOne<I, T: Relation>: HasOne<T> where T::Resource: RawHasPatch<Synchronous> {
    type PatchOneFut: IntoFuture<Item = ResourceResponse<I, T::Resource>, Error = Error>;
    fn patch_one(entity: &Entity<Self>, received: RawReceived<T::Resource, <T::Resource as RawHasPatch<Synchronous>>::Patch>) -> Self::PatchOneFut;
}

impl<I, T, Rel> PatchOne<I, Rel> for T
where T:                HasOne<Rel>,
      Rel:              Relation,
      Rel::Resource:    RawPatch<I> + RawHasPatch<Synchronous> {
    type PatchOneFut = Result<ResourceResponse<I, Rel::Resource>, Error>;
    fn patch_one(entity: &Entity<Self>, received: RawReceived<Rel::Resource, <Rel::Resource as RawHasPatch<Synchronous>>::Patch>) -> Self::PatchOneFut {
        if let Some(id) = <T as HasOne<Rel>>::has_one(entity).into_future().wait()? {
            <Rel::Resource as RawPatch<I>>::patch(id, received).into_future().wait()
        } else { Err(Error::NotFound) }
    }
}
