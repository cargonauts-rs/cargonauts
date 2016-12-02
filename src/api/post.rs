use std::sync::Arc;

use api::{Resource, Entity, Error};
use api::raw::{RawResource, RawReceived, ResourceResponse, CollectionResponse, ResourceObject};
use _internal::_UpdateRels;
use IntoFuture;
use futures::Future;
use futures::stream::{self, Stream};

pub trait Post: Resource {
    type PostFut: IntoFuture<Item = Self, Error = Error>;
    fn post(data: Self) -> Self::PostFut;
    fn append(data: Vec<Self>) -> Box<Future<Item = Vec<Self>, Error = Error>> {
        Box::new(stream::iter(data.into_iter().map(Ok)).and_then(Self::post).collect())
    }
}

pub trait RawPost<I>: RawResource {
    type RawPostFut: Future<Item = ResourceResponse<I, Self>, Error = Error>;
    type RawAppendFut: Future<Item = CollectionResponse<I, Self>, Error = Error>;
    fn post(received: RawReceived<Self, Self>) -> Self::RawPostFut;
    fn append(received: Vec<RawReceived<Self, Self>>) -> Self::RawAppendFut;
}

impl<I, T> RawPost<I> for T where T: Post + _UpdateRels, I: 'static {
    type RawPostFut = Box<Future<Item = ResourceResponse<I, T>, Error = Error>>;
    type RawAppendFut = Box<Future<Item = CollectionResponse<I, T>, Error = Error>>;

    fn post(received: RawReceived<Self, Self>) -> Self::RawPostFut {
        let RawReceived { attributes, relationships } = received;
        Box::new(<Self as Post>::post(attributes).into_future().and_then(move |resource| {
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

    fn append(received: Vec<RawReceived<Self, Self>>) -> Self::RawAppendFut {
        let (data, rels) = split(received.into_iter().map(|r| (r.attributes, r.relationships)));
        Box::new(<Self as Post>::append(data).into_future().and_then(move |data| {
            stream::iter(data.into_iter().zip(rels).map(Ok)).and_then(move |(resource, rels)| {
                let entity = Entity::Resource(Arc::new(resource));
                <T as _UpdateRels>::update_rels(entity.clone(), rels).into_future().join(Ok(entity))
            }).fold(CollectionResponse::default(), |mut response, (rels, entity)| {
                match entity {
                    Entity::Resource(resource)  => response.resources.push(ResourceObject {
                        id: resource.id(),
                        attributes: Arc::try_unwrap(resource).ok().unwrap(),
                        relationships: rels,
                    }),
                    _                           => unreachable!()
                }
                Ok(response)
            })
        }))
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
