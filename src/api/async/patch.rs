use Deserialize;
use api::async::{AsyncAction, AsyncJob};
use api::async::raw::JobResponse;
use api::{Resource, Error};
use api::raw::{RawUpdate, ResourceObject, NoRelationships};
use _internal::_UpdateRels;
use IntoFuture;
use futures::Future;

pub trait PatchAsync: AsyncAction + RawUpdate {
    type Patch: Deserialize;
    type PatchAsyncFut: IntoFuture<Item = Self::Job, Error = Error>;
    fn patch_async(id: &Self::Id, patch: Self::Patch) -> Self::PatchAsyncFut;
}

pub trait RawPatchAsync: AsyncAction + RawUpdate {
    type Patch: Deserialize;
    type RawPatchAsyncFut: IntoFuture<Item = JobResponse<Self>, Error = Error>;
    fn patch_async(id: Self::Id, patch: Self::Patch, rels: <Self as RawUpdate>::Relationships) -> Self::RawPatchAsyncFut;
}

impl<T> RawPatchAsync for T where T: PatchAsync + _UpdateRels {
    type Patch = <T as PatchAsync>::Patch;
    type RawPatchAsyncFut = Result<JobResponse<Self>, Error>;
    fn patch_async(id: Self::Id, patch: Self::Patch, rels: <Self as RawUpdate>::Relationships) -> Self::RawPatchAsyncFut {
        let mut job = <Self as PatchAsync>::patch_async(&id, patch).into_future().wait()?;
        job.cache_rels(rels)?;
        Ok(JobResponse {
            resource: ResourceObject {
                id: job.id(),
                attributes: job,
                relationships: NoRelationships,
            },
        })
    }
}
