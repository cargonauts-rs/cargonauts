use api::{Entity, Error};
use api::raw::{RawPost, RawReceived, ResourceResponse, CollectionResponse, RawResource};
use api::rel::{ToMany, PostLinks, ReplaceLinks};
use IntoFuture;
use futures::Future;

pub trait PostMany<I, Rel: ToMany>: PostLinks<Rel> where Rel::Resource: RawResource {
    type PostOneFut: Future<Item = ResourceResponse<I, Rel::Resource>, Error = Error> + 'static;
    type AppendManyFut: Future<Item = CollectionResponse<I, Rel::Resource>, Error = Error> + 'static;
    fn post_one(entity: Entity<Self>, received: RawReceived<Rel::Resource, Rel::Resource>) -> Self::PostOneFut;
    fn append_many(entity: Entity<Self>, received: Vec<RawReceived<Rel::Resource, Rel::Resource>>) -> Self::AppendManyFut;
}

impl<I, T, Rel> PostMany<I, Rel> for T
where T:                PostLinks<Rel>,
      Rel:              ToMany,
      Rel::Resource:    RawPost<I>,
      I:                'static,
{
    type PostOneFut = Box<Future<Item = ResourceResponse<I, Rel::Resource>, Error = Error>>;
    type AppendManyFut = Box<Future<Item = CollectionResponse<I, Rel::Resource>, Error = Error>>;
    fn post_one(entity: Entity<Self>, received: RawReceived<Rel::Resource, Rel::Resource>) -> Self::PostOneFut {
        Box::new(RawPost::post(received).into_future().and_then(move |response| {
            <T as PostLinks<Rel>>::post_link(entity, response.resource.id.clone()).into_future().map(|_| response)
        }))
    }
    fn append_many(entity: Entity<Self>, received: Vec<RawReceived<Rel::Resource, Rel::Resource>>) -> Self::AppendManyFut {
        Box::new(RawPost::append(received).into_future().and_then(move |response| {
            let ids = response.resources.iter().map(|resource| resource.id.clone()).collect::<Vec<_>>();
            <T as PostLinks<Rel>>::append_links(entity, ids).into_future().map(|_| response)
        }))
    }
}

pub trait ReplaceMany<I, Rel: ToMany>: ReplaceLinks<Rel> where Rel::Resource: RawResource {
    type ReplaceManyFut: Future<Item = CollectionResponse<I, Rel::Resource>, Error = Error> + 'static;
    fn replace_many(entity: Entity<Self>, received: Vec<RawReceived<Rel::Resource, Rel::Resource>>) -> Self::ReplaceManyFut;
}

impl<I, T, Rel> ReplaceMany<I, Rel> for T
where
    T: ReplaceLinks<Rel>,
    Rel: ToMany,
    Rel::Resource: RawPost<I>,
    I: 'static,
{
    type ReplaceManyFut = Box<Future<Item = CollectionResponse<I, Rel::Resource>, Error = Error>>;
    fn replace_many(entity: Entity<Self>, received: Vec<RawReceived<Rel::Resource, Rel::Resource>>) -> Self::ReplaceManyFut {
        Box::new(RawPost::append(received).into_future().and_then(move |response| {
            let ids = response.resources.iter().map(|resource| resource.id.clone()).collect::<Vec<_>>();
            <T as ReplaceLinks<Rel>>::replace_links(entity, &ids).into_future().map(|_| response)
        }))
    }
}
