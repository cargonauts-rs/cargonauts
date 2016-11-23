use api::{Resource, Entity, Error};
use api::raw::{RawUpdate, RawReceived, ResourceResponse, ResourceObject};
use _internal::_UpdateRels;
use IntoFuture;
use futures::Future;

pub trait Post: Resource {
    type PostFut: IntoFuture<Item = Self, Error = Error>;
    fn post(data: Self) -> Self::PostFut;
}

pub trait RawPost<I>: RawUpdate {
    type RawPostFut: Future<Item = ResourceResponse<I, Self>, Error = Error>;
    fn post(received: RawReceived<Self, Self>) -> Self::RawPostFut;
}

impl<I, T> RawPost<I> for T where T: Post + _UpdateRels, I: 'static {
    type RawPostFut = Box<Future<Item = ResourceResponse<I, T>, Error = Error>>;
    fn post(received: RawReceived<Self, Self>) -> Self::RawPostFut {
        let RawReceived { attributes, relationships } = received;
        Box::new(<Self as Post>::post(attributes).into_future().and_then(move |resource| {
            let entity = Entity::Resource(resource);
            <T as _UpdateRels>::update_rels(&entity, relationships).map(move |relationships| {
                let resource = match entity {
                    Entity::Resource(resource)  => resource,
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
