use api::{Entity, Error};
use api::raw::{RawPost, PostResponse, RawUpdate};
use api::rel::{Relation, LinkOne, AppendLinks};
use Deserialize;
use IntoFuture;
use futures::Future;

pub trait PostOne<Rel: Relation>: LinkOne<Rel> where Rel::Resource: RawUpdate + Deserialize {
    type PostOneFut: IntoFuture<Item = PostResponse<Rel::Resource>, Error = Error>;
    fn post_one(entity: &Entity<Self>, post: Rel::Resource, rels: <Rel::Resource as RawUpdate>::Relationships) -> Self::PostOneFut;
}

impl<T, Rel> PostOne<Rel> for T
where T:                LinkOne<Rel>,
      Rel:              Relation,
      Rel::Resource:    RawPost,
{
    type PostOneFut = Result<PostResponse<Rel::Resource>, Error>;
    fn post_one(entity: &Entity<Self>, post: Rel::Resource, rels: <Rel::Resource as RawUpdate>::Relationships) -> Self::PostOneFut {
        let response = RawPost::post(post, rels).into_future().wait()?;
        <T as LinkOne<Rel>>::link_one(entity, &response.resource.id).into_future().wait()?;
        Ok(response)
    }
}

pub trait Append<Rel: Relation>: AppendLinks<Rel> where Rel::Resource: RawUpdate + Deserialize {
    type AppendFut: IntoFuture<Item = PostResponse<Rel::Resource>, Error = Error>;
    fn append(entity: &Entity<Self>, post: Rel::Resource, rels: <Rel::Resource as RawUpdate>::Relationships) -> Self::AppendFut;
}

impl<T, Rel> Append<Rel> for T
where T:                AppendLinks<Rel>,
      Rel:              Relation,
      Rel::Resource:    RawPost,
{
    type AppendFut = Result<PostResponse<Rel::Resource>, Error>;
    fn append(entity: &Entity<Self>, post: Rel::Resource, rels: <Rel::Resource as RawUpdate>::Relationships) -> Self::AppendFut {
        let response: PostResponse<Rel::Resource> = RawPost::post(post, rels).into_future().wait()?;
        let rel_id = response.resource.id.clone();
        <T as AppendLinks<Rel>>::append_links(entity, &[rel_id]).into_future().wait()?;
        Ok(response)
    }
}
