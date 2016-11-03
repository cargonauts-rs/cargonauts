use Deserialize;
use api::{AsyncJob, Resource, Result, Entity};
use api::raw::{RawUpdate, ResourceObject, RawFetch, NoRelationships};
use _internal::_UpdateRels;

pub trait Post: Resource + Deserialize {
    fn post(self) -> Result<Self>;
}

pub trait PostAsync: RawUpdate + Deserialize {
    type Job: AsyncJob<Self>;
    fn post_async(self) -> Result<Self::Job>;
}

pub trait RawPost: RawUpdate + Deserialize {
    fn post(self, rels: <Self as RawUpdate>::Relationships) -> Result<PostResponse<Self>>;
}

pub trait RawPostAsync: RawUpdate + Deserialize {
    type Job: AsyncJob<Self>;
    fn post_async(self, rels: <Self as RawUpdate>::Relationships) -> Result<PostResponse<Self::Job>>;
}

impl<T> RawPost for T where T: Post + _UpdateRels {
    fn post(self, rels: <T as RawUpdate>::Relationships) -> Result<PostResponse<T>> {
        let entity = Entity::Resource(<Self as Post>::post(self)?);
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
    fn post_async(self, rels: <Self as RawUpdate>::Relationships) -> Result<PostResponse<Self::Job>> {
        let mut job = <Self as PostAsync>::post_async(self)?;
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

