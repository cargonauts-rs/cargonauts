use Deserialize;
use api::{AsyncAction, AsyncJob, Resource, Entity, Error};
use api::raw::{RawUpdate, ResourceResponse, JobResponse, ResourceObject, NoRelationships};
use _internal::_UpdateRels;
use IntoFuture;
use futures::Future;

pub trait Post: Resource + Deserialize {
    type PostFut: IntoFuture<Item = Self, Error = Error>;
    fn post(self) -> Self::PostFut;
}

pub trait PostAsync: AsyncAction + RawUpdate + Deserialize {
    type PostAsyncFut: IntoFuture<Item = Self::Job, Error = Error>;
    fn post_async(self) -> Self::PostAsyncFut;
}

pub trait RawPost<I>: RawUpdate + Deserialize {
    type RawPostFut: IntoFuture<Item = ResourceResponse<I, Self>, Error = Error>;
    fn post(self, rels: <Self as RawUpdate>::Relationships) -> Self::RawPostFut;
}

pub trait RawPostAsync: AsyncAction + RawUpdate + Deserialize {
    type RawPostAsyncFut: IntoFuture<Item = JobResponse<Self>, Error = Error>;
    fn post_async(self, rels: <Self as RawUpdate>::Relationships) -> Self::RawPostAsyncFut;
}

impl<I, T> RawPost<I> for T where T: Post + _UpdateRels {
    type RawPostFut = Result<ResourceResponse<I, T>, Error>;
    fn post(self, rels: <T as RawUpdate>::Relationships) -> Self::RawPostFut {
        let entity = Entity::Resource(<Self as Post>::post(self).into_future().wait()?);
        let relationships = <T as _UpdateRels>::update_rels(&entity, rels)?;
        let resource = match entity {
            Entity::Resource(resource)  => resource,
            _                           => unreachable!()
        };
        Ok(ResourceResponse {
            resource: ResourceObject {
                id: resource.id(),
                attributes: resource,
                relationships: relationships,
            },
            includes: vec![],
        })
    }
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
