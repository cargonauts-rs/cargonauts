use api::{Entity, Error};
use api::raw::{RawPost, ResourceResponse, RawUpdate};
use api::rel::{Relation, LinkOne, AppendLinks};
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

pub trait Append<I, Rel: Relation>: AppendLinks<Rel> where Rel::Resource: RawUpdate + Deserialize {
    type AppendFut: IntoFuture<Item = ResourceResponse<I, Rel::Resource>, Error = Error>;
    fn append(entity: &Entity<Self>, post: Rel::Resource, rels: <Rel::Resource as RawUpdate>::Relationships) -> Self::AppendFut;
}

impl<I, T, Rel> Append<I, Rel> for T
where T:                AppendLinks<Rel>,
      Rel:              Relation,
      Rel::Resource:    RawPost<I>,
{
    type AppendFut = Result<ResourceResponse<I, Rel::Resource>, Error>;
    fn append(entity: &Entity<Self>, post: Rel::Resource, rels: <Rel::Resource as RawUpdate>::Relationships) -> Self::AppendFut {
        let response: ResourceResponse<I, Rel::Resource> = RawPost::post(post, rels).into_future().wait()?;
        let rel_id = response.resource.id.clone();
        <T as AppendLinks<Rel>>::append_links(entity, &[rel_id]).into_future().wait()?;
        Ok(response)
    }
}
