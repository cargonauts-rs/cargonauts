use api::{Entity, Error};
use api::raw::{RawPost, RawReceived, CollectionResponse, RawResource};
use api::rel::{ToMany, AppendLinks, ReplaceLinks};
use IntoFuture;
use futures::Future;

pub trait AppendMany<I, Rel: ToMany>: AppendLinks<Rel> where Rel::Resource: RawResource {
    type AppendManyFut: Future<Item = CollectionResponse<I, Rel::Resource>, Error = Error> + 'static;
    fn append_many(entity: Entity<Self>, received: Vec<RawReceived<Rel::Resource, Rel::Resource>>) -> Self::AppendManyFut;
}

impl<I, T, Rel> AppendMany<I, Rel> for T
where T:                AppendLinks<Rel>,
      Rel:              ToMany,
      Rel::Resource:    RawPost<I>,
      I:                'static,
{
    type AppendManyFut = Box<Future<Item = CollectionResponse<I, Rel::Resource>, Error = Error>>;
    fn append_many(entity: Entity<Self>, received: Vec<RawReceived<Rel::Resource, Rel::Resource>>) -> Self::AppendManyFut {
        Box::new(RawPost::append(received).into_future().and_then(move |response| {
            let ids = response.resources.iter().map(|resource| resource.id.clone()).collect::<Vec<_>>();
            <T as AppendLinks<Rel>>::append_links(entity, &ids).into_future().map(|_| response)
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
