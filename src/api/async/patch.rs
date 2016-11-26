use api::async::{AsyncAction, AsyncJob};
use api::async::raw::JobResponse;
use api::{Resource, Error};
use api::raw::{RawResource, RawHasPatch, RawReceived, ResourceObject, NoRelationships};
use _internal::_UpdateRels;
use IntoFuture;
use futures::Future;

pub trait PatchAsync: AsyncAction + Resource {
    type Patch;
    type PatchAsyncFut: IntoFuture<Item = Self::Job, Error = Error>;
    fn patch_async(id: &Self::Id, patch: Self::Patch) -> Self::PatchAsyncFut;
}

pub trait RawPatchAsync: RawHasPatch<Asynchronous> + AsyncAction + RawResource {
    type RawPatchAsyncFut: Future<Item = JobResponse<Self>, Error = Error>;
    fn patch_async(id: Self::Id, received: RawReceived<Self, Self::Patch>) -> Self::RawPatchAsyncFut;
}

pub struct Asynchronous;

impl<T> RawHasPatch<Asynchronous> for T where T: PatchAsync + _UpdateRels {
    type Patch = <T as PatchAsync>::Patch;
}

impl<T> RawPatchAsync for T where T: PatchAsync + _UpdateRels {
    type RawPatchAsyncFut = Box<Future<Item = JobResponse<Self>, Error = Error>>;
    fn patch_async(id: Self::Id, received: RawReceived<Self, Self::Patch>) -> Self::RawPatchAsyncFut {
        let RawReceived { attributes, relationships } = received;
        Box::new(<Self as PatchAsync>::patch_async(&id, attributes).into_future().and_then(|mut job| {
            job.cache_rels(relationships)?;
            Ok(JobResponse {
                resource: ResourceObject {
                    id: job.id(),
                    attributes: job,
                    relationships: NoRelationships,
                },
            })
        }))
    }
}
