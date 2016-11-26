use api::async::{AsyncAction, AsyncJob};
use api::async::raw::JobResponse;
use api::{Resource, Error};
use api::raw::{RawResource, RawReceived, ResourceObject, NoRelationships};
use _internal::_UpdateRels;
use IntoFuture;
use futures::Future;

pub trait PostAsync: AsyncAction + Resource {
    type PostAsyncFut: IntoFuture<Item = Self::Job, Error = Error>;
    fn post_async(data: Self) -> Self::PostAsyncFut;
}

pub trait RawPostAsync: AsyncAction + RawResource {
    type RawPostAsyncFut: Future<Item = JobResponse<Self>, Error = Error>;
    fn post_async(received: RawReceived<Self, Self>) -> Self::RawPostAsyncFut;
}

impl<T> RawPostAsync for T where T: PostAsync + _UpdateRels {
    type RawPostAsyncFut = Box<Future<Item = JobResponse<Self>, Error = Error>>;
    fn post_async(received: RawReceived<Self, Self>) -> Self::RawPostAsyncFut {
        let RawReceived { attributes, relationships } = received;
        Box::new(<Self as PostAsync>::post_async(attributes).into_future().and_then(|mut job| {
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
