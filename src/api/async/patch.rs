use api::async::{AsyncAction, AsyncJob};
use api::async::raw::JobResponse;
use api::{Resource, Error};
use api::raw::{RawUpdate, RawHasPatch, RawReceived, ResourceObject, NoRelationships};
use _internal::_UpdateRels;
use IntoFuture;
use futures::Future;

pub trait PatchAsync: AsyncAction + RawUpdate {
    type Patch;
    type PatchAsyncFut: IntoFuture<Item = Self::Job, Error = Error>;
    fn patch_async(id: &Self::Id, patch: Self::Patch) -> Self::PatchAsyncFut;
}

pub trait RawPatchAsync: RawHasPatch<Asynchronous> + AsyncAction + RawUpdate {
    type RawPatchAsyncFut: IntoFuture<Item = JobResponse<Self>, Error = Error>;
    fn patch_async(id: Self::Id, received: RawReceived<Self, Self::Patch>) -> Self::RawPatchAsyncFut;
}

pub struct Asynchronous;

impl<T> RawHasPatch<Asynchronous> for T where T: PatchAsync + _UpdateRels {
    type Patch = <T as PatchAsync>::Patch;
}

impl<T> RawPatchAsync for T where T: PatchAsync + _UpdateRels {
    type RawPatchAsyncFut = Result<JobResponse<Self>, Error>;
    fn patch_async(id: Self::Id, received: RawReceived<Self, Self::Patch>) -> Self::RawPatchAsyncFut {
        let mut job = <Self as PatchAsync>::patch_async(&id, received.attributes).into_future().wait()?;
        job.cache_rels(received.relationships)?;
        Ok(JobResponse {
            resource: ResourceObject {
                id: job.id(),
                attributes: job,
                relationships: NoRelationships,
            },
        })
    }
}
