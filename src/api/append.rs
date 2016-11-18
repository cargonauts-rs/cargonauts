use api::{Resource, Entity, Error};
use api::raw::{RawUpdate, RawReceived, CollectionResponse, ResourceObject};
use _internal::_UpdateRels;
use IntoFuture;
use futures::Future;

pub trait Append: Resource {
    type AppendFut: IntoFuture<Item = Vec<Self>, Error = Error>;
    fn append(data: Vec<Self>) -> Self::AppendFut;
}

pub trait RawAppend<I>: RawUpdate {
    type RawAppendFut: IntoFuture<Item = CollectionResponse<I, Self>, Error = Error>;
    fn append(received: Vec<RawReceived<Self, Self>>) -> Self::RawAppendFut;
}

impl<I, T> RawAppend<I> for T where T: Append + _UpdateRels {
    type RawAppendFut = Result<CollectionResponse<I, T>, Error>;
    fn append(received: Vec<RawReceived<Self, Self>>) -> Self::RawAppendFut {
        let (data, rels) = split(received.into_iter().map(|r| (r.attributes, r.relationships)));
        let mut resources = vec![];
        for (resource, rels) in <Self as Append>::append(data).into_future().wait()?.into_iter().zip(rels) {
            let entity = Entity::Resource(resource);
            let relationships = <T as _UpdateRels>::update_rels(&entity, rels)?;
            match entity {
                Entity::Resource(resource)  => resources.push(ResourceObject {
                    id: resource.id(),
                    attributes: resource,
                    relationships: relationships,
                }),
                _                           => unreachable!()
            }
        }
        Ok(CollectionResponse {
            resources: resources,
            includes: vec![],
        })
    }
}

fn split<I: IntoIterator<Item = (T, U)>, T, U>(iter: I) -> (Vec<T>, Vec<U>) {
    let (mut vec1, mut vec2) = (vec![], vec![]);
    for (left, right) in iter {
        vec1.push(left);
        vec2.push(right);
    }
    (vec1, vec2)
}

