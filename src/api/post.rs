use Deserialize;
use api::{AsyncJob, Resource, Result, Entity};
use api::raw::{RawUpdate, ResourceObject, RawFetch};
use _internal::_UpdateRels;

pub trait Post: Resource where Self::Repr: Deserialize {
    fn post(repr: Self::Repr) -> Result<Self>;
}

pub trait PostAsync: RawUpdate where Self::Repr: Deserialize {
    type Job: AsyncJob<Self>;
    fn post_async(repr: Self::Repr) -> Result<Self::Job>;
}

pub trait RawPost: RawUpdate where Self::Repr: Deserialize {
    fn post(repr: Self::Repr, rels: <Self as RawUpdate>::Relationships) -> Result<PostResponse<Self>>;
}

pub trait RawPostAsync: RawUpdate where Self::Repr: Deserialize {
    type Job: AsyncJob<Self>;
    fn post_async(repr: Self::Repr, rels: <Self as RawUpdate>::Relationships) -> Result<PostResponse<Self::Job>>;
}

impl<T> RawPost for T where T: Post + _UpdateRels, T::Repr: Deserialize {
    fn post(repr: Self::Repr, rels: <T as RawUpdate>::Relationships) -> Result<PostResponse<T>> {
        let entity = Entity::Resource(<Self as Post>::post(repr)?);
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

impl<T> RawPostAsync for T where T: PostAsync + _UpdateRels, T::Repr: Deserialize {
    type Job = <T as PostAsync>::Job;
    fn post_async(repr: Self::Repr, rels: <Self as RawUpdate>::Relationships) -> Result<PostResponse<Self::Job>> {
        let mut job = <Self as PostAsync>::post_async(repr)?;
        job.cache_rels(rels)?;
        Ok(PostResponse {
            resource: ResourceObject {
                id: job.id(),
                attributes: job,
                relationships: (),
            }
        })
    }
}

pub struct PostResponse<T> where T: RawFetch {
    pub resource: ResourceObject<T>,
}

