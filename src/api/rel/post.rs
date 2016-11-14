use api::{Entity, Error};
use api::raw::{RawPost, ResourceResponse, CollectionResponse, RawUpdate};
use api::rel::{Relation, LinkOne, AppendLinks, ReplaceLinks};
use Deserialize;
use IntoFuture;
use futures::Future;

pub trait PostOne<I, Rel: Relation>: LinkOne<Rel> where Rel::Resource: RawUpdate + Deserialize {
    type PostOneFut: IntoFuture<Item = ResourceResponse<I, Rel::Resource>, Error = Error>;
    fn post_one(entity: &Entity<Self>, post: Rel::Resource, rels: <Rel::Resource as RawUpdate>::Relationships) -> Self::PostOneFut;
}

impl<I, T, Rel> PostOne<I, Rel> for T
where T:                LinkOne<Rel>,
      Rel:              Relation,
      Rel::Resource:    RawPost<I>,
{
    type PostOneFut = Result<ResourceResponse<I, Rel::Resource>, Error>;
    fn post_one(entity: &Entity<Self>, post: Rel::Resource, rels: <Rel::Resource as RawUpdate>::Relationships) -> Self::PostOneFut {
        let response = RawPost::post(post, rels).into_future().wait()?;
        <T as LinkOne<Rel>>::link_one(entity, &response.resource.id).into_future().wait()?;
        Ok(response)
    }
}

pub trait AppendMany<I, Rel: Relation>: AppendLinks<Rel> where Rel::Resource: RawUpdate + Deserialize {
    type AppendManyFut: IntoFuture<Item = CollectionResponse<I, Rel::Resource>, Error = Error>;
    fn append(entity: &Entity<Self>, post: Vec<(Rel::Resource, <Rel::Resource as RawUpdate>::Relationships)>) -> Self::AppendManyFut;
}

impl<I, T, Rel> AppendMany<I, Rel> for T
where T:                AppendLinks<Rel>,
      Rel:              Relation,
      Rel::Resource:    RawPost<I>,
{
    type AppendManyFut = Result<CollectionResponse<I, Rel::Resource>, Error>;
    fn append(entity: &Entity<Self>, post: Vec<(Rel::Resource, <Rel::Resource as RawUpdate>::Relationships)>) -> Self::AppendManyFut {
        let mut resources = vec![];
        for (post, rels) in post {
            let response = RawPost::post(post, rels).into_future().wait()?;
            resources.push(response.resource);
        }
        let ids = resources.iter().map(|resource| resource.id.clone()).collect::<Vec<_>>();
        <T as AppendLinks<Rel>>::append_links(entity, &ids).into_future().wait()?;
        Ok(CollectionResponse {
            resources: resources,
            includes: vec![],
        })
    }
}

pub trait ReplaceMany<I, Rel: Relation>: ReplaceLinks<Rel> where Rel::Resource: RawUpdate + Deserialize {
    type ReplaceManyFut: IntoFuture<Item = CollectionResponse<I, Rel::Resource>, Error = Error>;
    fn replace(entity: &Entity<Self>, post: Vec<(Rel::Resource, <Rel::Resource as RawUpdate>::Relationships)>) -> Self::ReplaceManyFut;
}

impl<I, T, Rel> ReplaceMany<I, Rel> for T
where
    T: ReplaceLinks<Rel>,
    Rel: Relation,
    Rel::Resource: RawPost<I>,
{
    type ReplaceManyFut = Result<CollectionResponse<I, Rel::Resource>, Error>;
    fn replace(entity: &Entity<Self>, post: Vec<(Rel::Resource, <Rel::Resource as RawUpdate>::Relationships)>) -> Self::ReplaceManyFut {
        let mut resources = vec![];
        for (post, rels) in post {
            let response = RawPost::post(post, rels).into_future().wait()?;
            resources.push(response.resource);
        }
        let ids = resources.iter().map(|resource| resource.id.clone()).collect::<Vec<_>>();
        <T as ReplaceLinks<Rel>>::replace_links(entity, &ids).into_future().wait()?;
        Ok(CollectionResponse {
            resources: resources,
            includes: vec![],
        })
    }
}
