use Deserialize;
use api::{AsyncJob, Resource, Error, Entity};
use api::raw::{RawUpdate, ResourceObject, RawFetch, NoRelationships};
use _internal::_UpdateRels;
use IntoFuture;
use futures::Future;

pub trait Patch: Resource {
    type Patch: Deserialize;
    type PatchFut: IntoFuture<Item = Self, Error = Error>;
    fn patch(id: &Self::Id, patch: Self::Patch) -> Self::PatchFut;
}

pub trait RawPatch: RawUpdate {
    type Patch: Deserialize;
    type RawPatchFut: IntoFuture<Item = PatchResponse<Self>, Error = Error>;
    fn patch(id: Self::Id, patch: Self::Patch, rels: <Self as RawUpdate>::Relationships) -> Self::RawPatchFut;
}

pub trait PatchAsync: RawUpdate {
    type Patch: Deserialize;
    type Job: AsyncJob<Self>;
    type PatchAsyncFut: IntoFuture<Item = Self::Job, Error = Error>;
    fn patch_async(id: &Self::Id, patch: Self::Patch) -> Self::PatchAsyncFut;
}

pub trait RawPatchAsync: RawUpdate {
    type Patch: Deserialize;
    type Job: AsyncJob<Self>;
    type RawPatchAsyncFut: IntoFuture<Item = PatchResponse<Self::Job>, Error = Error>;
    fn patch_async(id: Self::Id, patch: Self::Patch, rels: <Self as RawUpdate>::Relationships) -> Self::RawPatchAsyncFut;
}

impl<T> RawPatch for T where T: Patch + _UpdateRels {
    type Patch = <Self as Patch>::Patch;
    type RawPatchFut = Result<PatchResponse<Self>, Error>;
    fn patch(id: Self::Id, patch: Self::Patch, rels: <Self as RawUpdate>::Relationships) -> Self::RawPatchFut {
        let entity = Entity::Resource(<T as Patch>::patch(&id, patch).into_future().wait()?);
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
    type RawPatchAsyncFut = Result<PatchResponse<Self::Job>, Error>;
    fn patch_async(id: Self::Id, patch: Self::Patch, rels: <Self as RawUpdate>::Relationships) -> Self::RawPatchAsyncFut {
        let mut job = <Self as PatchAsync>::patch_async(&id, patch).into_future().wait()?;
        job.cache_rels(rels)?;
        Ok(PatchResponse {
            resource: ResourceObject {
                id: job.id(),
                attributes: job,
                relationships: NoRelationships,
            }
        })
    }
}

pub struct PatchResponse<T: RawFetch> {
    pub resource: ResourceObject<T>,
}
