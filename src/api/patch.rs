use Deserialize;
use api::{AsyncJob, Resource, Error, Entity};
use api::raw::{RawUpdate, ResourceObject, RawFetch};
use _internal::_UpdateRels;

pub trait Patch: Resource {
    type Patch: Deserialize;
    fn patch(id: &Self::Id, patch: Self::Patch) -> Result<Self, Error>;
}

pub trait RawPatch: RawUpdate {
    type Patch: Deserialize;
    fn patch(id: Self::Id, patch: Self::Patch, rels: <Self as RawUpdate>::Relationships) -> Result<PatchResponse<Self>, Error>;
}

pub trait PatchAsync: RawUpdate {
    type Patch: Deserialize;
    type Job: AsyncJob<Self>;
    fn patch_async(id: &Self::Id, patch: Self::Patch) -> Result<Self::Job, Error>;
}

pub trait RawPatchAsync: RawUpdate {
    type Patch: Deserialize;
    type Job: AsyncJob<Self>;
    fn patch_async(id: Self::Id, patch: Self::Patch, rels: <Self as RawUpdate>::Relationships) -> Result<PatchResponse<Self::Job>, Error>;
}

impl<T> RawPatch for T where T: Patch + _UpdateRels {
    type Patch = <Self as Patch>::Patch;
    fn patch(id: Self::Id, patch: Self::Patch, rels: <Self as RawUpdate>::Relationships) -> Result<PatchResponse<Self>, Error> {
        let entity = Entity::Resource(<T as Patch>::patch(&id, patch)?);
        let relationships = <T as _UpdateRels>::update_rels(&entity, rels)?;
        let resource = match entity {
            Entity::Resource(resource)  => resource,
            _                           => unreachable!()
        };
        Ok(PatchResponse {
            resource: ResourceObject {
                id: resource.id(),
                attributes: resource,
                relationships: relationships,
            }
        })
    }
}

impl<T> RawPatchAsync for T where T: PatchAsync + _UpdateRels {
    type Patch = <T as PatchAsync>::Patch;
    type Job = <T as PatchAsync>::Job;
    fn patch_async(id: Self::Id, patch: Self::Patch, rels: <Self as RawUpdate>::Relationships) -> Result<PatchResponse<Self::Job>, Error> {
        let mut job = <Self as PatchAsync>::patch_async(&id, patch)?;
        job.cache_rels(rels)?;
        Ok(PatchResponse {
            resource: ResourceObject {
                id: job.id(),
                attributes: job,
                relationships: (),
            }
        })
    }
}

pub struct PatchResponse<T: RawFetch> {
    pub resource: ResourceObject<T>,
}
