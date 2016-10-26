use api::{Entity, Resource, Result};
use api::raw::{RawPost, PostResponse, RawUpdate};
use api::rel::{Relation, LinkOne, AppendLinks};
use Deserialize;

pub trait PostOne<Rel: Relation>: LinkOne<Rel> where Rel::Resource: RawUpdate, <Rel::Resource as Resource>::Repr: Deserialize {
    fn post_one(entity: &Entity<Self>, post: <Rel::Resource as Resource>::Repr, rels: <Rel::Resource as RawUpdate>::Relationships) -> Result<PostResponse<Rel::Resource>>;
}

impl<T, Rel> PostOne<Rel> for T
where T:                LinkOne<Rel>,
      Rel:              Relation,
      Rel::Resource:    RawPost,
      <Rel::Resource as Resource>::Repr: Deserialize {
    fn post_one(entity: &Entity<Self>, post: <Rel::Resource as Resource>::Repr, rels: <Rel::Resource as RawUpdate>::Relationships) -> Result<PostResponse<Rel::Resource>> {
        let response = RawPost::post(post, rels)?;
        <T as LinkOne<Rel>>::link_one(entity, &response.resource.id)?;
        Ok(response)
    }
}

pub trait Append<Rel: Relation>: AppendLinks<Rel> where Rel::Resource: RawUpdate, <Rel::Resource as Resource>::Repr: Deserialize {
    fn append(entity: &Entity<Self>, post: <Rel::Resource as Resource>::Repr, rels: <Rel::Resource as RawUpdate>::Relationships) -> Result<PostResponse<Rel::Resource>>;
}

impl<T, Rel> Append<Rel> for T
where T:                AppendLinks<Rel>,
      Rel:              Relation,
      Rel::Resource:    RawPost,
      <Rel::Resource as Resource>::Repr: Deserialize {
    fn append(entity: &Entity<Self>, post: <Rel::Resource as Resource>::Repr, rels: <Rel::Resource as RawUpdate>::Relationships) -> Result<PostResponse<Rel::Resource>> {
        let response: PostResponse<Rel::Resource> = RawPost::post(post, rels)?;
        let rel_id = response.resource.id.clone();
        <T as AppendLinks<Rel>>::append_links(entity, &[rel_id])?;
        Ok(response)
    }
}
