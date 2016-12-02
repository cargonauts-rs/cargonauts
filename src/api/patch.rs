use std::sync::Arc;

use api::{Resource, Error, Entity};
use api::raw::{ResourceResponse, RawReceived, RawResource, ResourceObject};
use _internal::_UpdateRels;
use IntoFuture;
use futures::Future;

pub trait Patch: Resource {
    type Patch;
    type PatchFut: IntoFuture<Item = Self, Error = Error>;
    fn patch(id: &Self::Id, patch: Self::Patch) -> Self::PatchFut;
}

pub trait RawHasPatch<Synchronicity>: RawResource {
    type Patch;
}

pub trait RawPatch<I>: RawHasPatch<Synchronous> {
    type RawPatchFut: Future<Item = ResourceResponse<I, Self>, Error = Error>;
    fn patch(id: Self::Id, received: RawReceived<Self, Self::Patch>) -> Self::RawPatchFut;
}

pub struct Synchronous;

impl<T> RawHasPatch<Synchronous> for T where T: Patch + _UpdateRels {
    type Patch = <T as Patch>::Patch;
}

impl<I, T> RawPatch<I> for T where T: Patch + _UpdateRels, I: 'static {
    type RawPatchFut = Box<Future<Item = ResourceResponse<I, Self>, Error = Error>>;
    fn patch(id: Self::Id, received: RawReceived<Self, Self::Patch>) -> Self::RawPatchFut {
        let RawReceived { attributes, relationships } = received;
        Box::new(<T as Patch>::patch(&id, attributes).into_future().and_then(move |resource| {
            let entity = Entity::Resource(Arc::new(resource));
            <T as _UpdateRels>::update_rels(entity.clone(), relationships).map(move |relationships| {
                let resource = match entity {
                    Entity::Resource(resource)  => Arc::try_unwrap(resource).ok().unwrap(),
                    _                           => unreachable!()
                };
                ResourceResponse {
                    resource: ResourceObject {
                        id: resource.id(),
                        attributes: resource,
                        relationships: relationships,
                    },
                    includes: vec![],
                }
            })
        }))
    }
}
