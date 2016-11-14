use Deserialize;
use api::async::{AsyncAction, AsyncJob};
use api::async::raw::JobResponse;
use api::{Resource, Error};
use api::raw::{RawUpdate, ResourceObject, NoRelationships};
use _internal::_UpdateRels;
use IntoFuture;
use futures::Future;

pub trait PostAsync: AsyncAction + RawUpdate + Deserialize {
    type PostAsyncFut: IntoFuture<Item = Self::Job, Error = Error>;
    fn post_async(self) -> Self::PostAsyncFut;
}

pub trait RawPostAsync: AsyncAction + RawUpdate + Deserialize {
    type RawPostAsyncFut: IntoFuture<Item = JobResponse<Self>, Error = Error>;
    fn post_async(self, rels: <Self as RawUpdate>::Relationships) -> Self::RawPostAsyncFut;
}

impl<T> RawPostAsync for T where T: PostAsync + _UpdateRels {
    type RawPostAsyncFut = Result<JobResponse<Self>, Error>;
    fn post_async(self, rels: <Self as RawUpdate>::Relationships) -> Self::RawPostAsyncFut {
        let mut job = <Self as PostAsync>::post_async(self).into_future().wait()?;
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
