use api::{Entity, Error};
use api::raw::{RawReceived, RawHasPatch, RawPatch, ResourceResponse, Synchronous};
use api::rel::{ToOne, HasOne};
use futures::Future;
use IntoFuture;

pub trait PatchOne<I, T: ToOne>: HasOne<T> where T::Resource: RawHasPatch<Synchronous> {
    type PatchOneFut: Future<Item = ResourceResponse<I, T::Resource>, Error = Error>;
    fn patch_one(entity: &Entity<Self>, received: RawReceived<T::Resource, <T::Resource as RawHasPatch<Synchronous>>::Patch>) -> Self::PatchOneFut;
}

impl<I, T, Rel> PatchOne<I, Rel> for T
where T:                HasOne<Rel>,
      Rel:              ToOne,
      Rel::Resource:    RawPatch<I> + RawHasPatch<Synchronous>,
      I:                'static {
    type PatchOneFut = Box<Future<Item = ResourceResponse<I, Rel::Resource>, Error = Error>>;
    fn patch_one(entity: &Entity<Self>, received: RawReceived<Rel::Resource, <Rel::Resource as RawHasPatch<Synchronous>>::Patch>) -> Self::PatchOneFut {
        Box::new(<T as HasOne<Rel>>::has_one(entity).into_future().and_then(|id| {
            if let Some(id) = id {
                Box::new(<Rel::Resource as RawPatch<I>>::patch(id, received)) as Box<Future<Item = ResourceResponse<I, Rel::Resource>, Error = Error>>
            } else { Box::new(Err(Error::NotFound).into_future()) }
        }))
    }
}
