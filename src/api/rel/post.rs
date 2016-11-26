use api::{Entity, Error};
use api::raw::{RawPost, RawAppend, RawReceived, ResourceResponse, CollectionResponse, RawUpdate};
use api::rel::{ToOne, ToMany, LinkOne, UnlinkOne, AppendLinks, ReplaceLinks};
use IntoFuture;
use futures::Future;

pub trait PostOne<I, Rel: ToOne>: LinkOne<Rel> + UnlinkOne<Rel> where Rel::Resource: RawUpdate {
    type PostOneFut: Future<Item = ResourceResponse<I, Rel::Resource>, Error = Error> + 'static;
    fn post_one(entity: Entity<Self>, received: RawReceived<Rel::Resource, Rel::Resource>) -> Self::PostOneFut;
}

impl<I, T, Rel> PostOne<I, Rel> for T
where T:                LinkOne<Rel> + UnlinkOne<Rel>,
      Rel:              ToOne,
      Rel::Resource:    RawPost<I>,
      I:                'static,
{
    type PostOneFut = Box<Future<Item = ResourceResponse<I, Rel::Resource>, Error = Error>>;
    fn post_one(entity: Entity<Self>, received: RawReceived<Rel::Resource, Rel::Resource>) -> Self::PostOneFut {
        Box::new(RawPost::post(received).into_future().and_then(move |response| {
            <T as LinkOne<Rel>>::link_one(&entity, &response.resource.id).into_future().map(|_| response)
        }))
    }
}

pub trait AppendMany<I, Rel: ToMany>: AppendLinks<Rel> where Rel::Resource: RawUpdate {
    type AppendManyFut: Future<Item = CollectionResponse<I, Rel::Resource>, Error = Error> + 'static;
    fn append_many(entity: Entity<Self>, received: Vec<RawReceived<Rel::Resource, Rel::Resource>>) -> Self::AppendManyFut;
}

impl<I, T, Rel> AppendMany<I, Rel> for T
where T:                AppendLinks<Rel>,
      Rel:              ToMany,
      Rel::Resource:    RawAppend<I>,
      I:                'static,
{
    type AppendManyFut = Box<Future<Item = CollectionResponse<I, Rel::Resource>, Error = Error>>;
    fn append_many(entity: Entity<Self>, received: Vec<RawReceived<Rel::Resource, Rel::Resource>>) -> Self::AppendManyFut {
        Box::new(RawAppend::append(received).into_future().and_then(move |response| {
            let ids = response.resources.iter().map(|resource| resource.id.clone()).collect::<Vec<_>>();
            <T as AppendLinks<Rel>>::append_links(&entity, &ids).into_future().map(|_| response)
        }))
    }
}

pub trait ReplaceMany<I, Rel: ToMany>: ReplaceLinks<Rel> where Rel::Resource: RawUpdate {
    type ReplaceManyFut: Future<Item = CollectionResponse<I, Rel::Resource>, Error = Error> + 'static;
    fn replace_many(entity: Entity<Self>, received: Vec<RawReceived<Rel::Resource, Rel::Resource>>) -> Self::ReplaceManyFut;
}

impl<I, T, Rel> ReplaceMany<I, Rel> for T
where
    T: ReplaceLinks<Rel>,
    Rel: ToMany,
    Rel::Resource: RawAppend<I>,
    I: 'static,
{
    type ReplaceManyFut = Box<Future<Item = CollectionResponse<I, Rel::Resource>, Error = Error>>;
    fn replace_many(entity: Entity<Self>, received: Vec<RawReceived<Rel::Resource, Rel::Resource>>) -> Self::ReplaceManyFut {
        Box::new(RawAppend::append(received).into_future().and_then(move |response| {
            let ids = response.resources.iter().map(|resource| resource.id.clone()).collect::<Vec<_>>();
            <T as ReplaceLinks<Rel>>::replace_links(&entity, &ids).into_future().map(|_| response)
        }))
    }
}
