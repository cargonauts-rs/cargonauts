use api::async::{AsyncAction, AsyncJob};
use api::async::raw::JobResponse;
use api::{Resource, Error};
use api::raw::{RawUpdate, RawReceived, ResourceObject, NoRelationships};
use _internal::_UpdateRels;
use IntoFuture;
use futures::Future;

pub trait PostAsync: AsyncAction + RawUpdate {
    type PostAsyncFut: IntoFuture<Item = Self::Job, Error = Error>;
    fn post_async(data: Self) -> Self::PostAsyncFut;
}

pub trait RawPostAsync: AsyncAction + RawUpdate {
    type RawPostAsyncFut: IntoFuture<Item = JobResponse<Self>, Error = Error>;
    fn post_async(received: RawReceived<Self, Self>) -> Self::RawPostAsyncFut;
}

impl<T> RawPostAsync for T where T: PostAsync + _UpdateRels {
    type RawPostAsyncFut = Result<JobResponse<Self>, Error>;
    fn post_async(received: RawReceived<Self, Self>) -> Self::RawPostAsyncFut {
        let mut job = <Self as PostAsync>::post_async(received.attributes).into_future().wait()?;
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
