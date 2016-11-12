use Deserialize;
use api::{AsyncAction, AsyncJob, Resource, Error, Entity};
use api::raw::{ResourceResponse, JobResponse, RawUpdate, ResourceObject, NoRelationships};
use _internal::_UpdateRels;
use IntoFuture;
use futures::Future;

pub trait Patch: Resource {
    type Patch: Deserialize;
    type PatchFut: IntoFuture<Item = Self, Error = Error>;
    fn patch(id: &Self::Id, patch: Self::Patch) -> Self::PatchFut;
}

pub trait RawPatch<I>: RawUpdate {
    type Patch: Deserialize;
    type RawPatchFut: IntoFuture<Item = ResourceResponse<I, Self>, Error = Error>;
    fn patch(id: Self::Id, patch: Self::Patch, rels: <Self as RawUpdate>::Relationships) -> Self::RawPatchFut;
}

pub trait PatchAsync: AsyncAction + RawUpdate {
    type Patch: Deserialize;
    type PatchAsyncFut: IntoFuture<Item = Self::Job, Error = Error>;
    fn patch_async(id: &Self::Id, patch: Self::Patch) -> Self::PatchAsyncFut;
}

pub trait RawPatchAsync: AsyncAction + RawUpdate {
    type Patch: Deserialize;
    type RawPatchAsyncFut: IntoFuture<Item = JobResponse<Self>, Error = Error>;
    fn patch_async(id: Self::Id, patch: Self::Patch, rels: <Self as RawUpdate>::Relationships) -> Self::RawPatchAsyncFut;
}

impl<I, T> RawPatch<I> for T where T: Patch + _UpdateRels {
    type Patch = <Self as Patch>::Patch;
    type RawPatchFut = Result<ResourceResponse<I, Self>, Error>;
    fn patch(id: Self::Id, patch: Self::Patch, rels: <Self as RawUpdate>::Relationships) -> Self::RawPatchFut {
        let entity = Entity::Resource(<T as Patch>::patch(&id, patch).into_future().wait()?);
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

impl<T> RawPatchAsync for T where T: PatchAsync + _UpdateRels {
    type Patch = <T as PatchAsync>::Patch;
    type RawPatchAsyncFut = Result<JobResponse<Self>, Error>;
    fn patch_async(id: Self::Id, patch: Self::Patch, rels: <Self as RawUpdate>::Relationships) -> Self::RawPatchAsyncFut {
        let mut job = <Self as PatchAsync>::patch_async(&id, patch).into_future().wait()?;
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
