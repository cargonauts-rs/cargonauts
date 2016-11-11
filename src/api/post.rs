use Deserialize;
use api::{AsyncJob, Resource, Entity, Error};
use api::raw::{RawUpdate, ResourceObject, RawFetch, NoRelationships};
use _internal::_UpdateRels;
use IntoFuture;
use futures::Future;

pub trait Post: Resource + Deserialize {
    type PostFut: IntoFuture<Item = Self, Error = Error>;
    fn post(self) -> Self::PostFut;
}

pub trait PostAsync: RawUpdate + Deserialize {
    type Job: AsyncJob<Self>;
    type PostAsyncFut: IntoFuture<Item = Self::Job, Error = Error>;
    fn post_async(self) -> Self::PostAsyncFut;
}

pub trait RawPost: RawUpdate + Deserialize {
    type RawPostFut: IntoFuture<Item = PostResponse<Self>, Error = Error>;
    fn post(self, rels: <Self as RawUpdate>::Relationships) -> Self::RawPostFut;
}

pub trait RawPostAsync: RawUpdate + Deserialize {
    type Job: AsyncJob<Self>;
    type RawPostAsyncFut: IntoFuture<Item = PostResponse<Self::Job>, Error = Error>;
    fn post_async(self, rels: <Self as RawUpdate>::Relationships) -> Self::RawPostAsyncFut;
}

impl<T> RawPost for T where T: Post + _UpdateRels {
    type RawPostFut = Result<PostResponse<T>, Error>;
    fn post(self, rels: <T as RawUpdate>::Relationships) -> Self::RawPostFut {
        let entity = Entity::Resource(<Self as Post>::post(self).into_future().wait()?);
        let relationships = <T as _UpdateRels>::update_rels(&entity, rels)?;
        let resource = match entity {
            Entity::Resource(resource)  => resource,
            _                           => unreachable!()
        };
        Ok(PostResponse {
            resource: ResourceObject {
                id: resource.id(),
                attributes: resource,
                relationships: relationships,
            }
        })
    }
}

impl<T> RawPostAsync for T where T: PostAsync + _UpdateRels {
    type Job = <T as PostAsync>::Job;
    type RawPostAsyncFut = Result<PostResponse<Self::Job>, Error>;
    fn post_async(self, rels: <Self as RawUpdate>::Relationships) -> Self::RawPostAsyncFut {
        let mut job = <Self as PostAsync>::post_async(self).into_future().wait()?;
        job.cache_rels(rels)?;
        Ok(PostResponse {
            resource: ResourceObject {
                id: job.id(),
                attributes: job,
                relationships: NoRelationships,
            }
        })
    }
}

pub struct PostResponse<T> where T: RawFetch {
    pub resource: ResourceObject<T>,
}

