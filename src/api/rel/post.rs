use api::Result;
use api::raw::{RawPost, PostResponse, RawUpdate};
use api::rel::{Relation, LinkOne, AppendLinks};
use Deserialize;

pub trait PostOne<Rel: Relation>: LinkOne<Rel> where Rel::Resource: RawUpdate + Deserialize {
    fn post_one(id: &Self::Id, post: Rel::Resource, rels: <Rel::Resource as RawUpdate>::Relationships) -> Result<PostResponse<Rel::Resource>>;
}

impl<T, Rel> PostOne<Rel> for T
where T:                LinkOne<Rel>,
      Rel:              Relation,
      Rel::Resource:    RawPost {
    fn post_one(id: &Self::Id, post: Rel::Resource, rels: <Rel::Resource as RawUpdate>::Relationships) -> Result<PostResponse<Rel::Resource>> {
        let response = try!(post.post(rels));
        try!(<T as LinkOne<Rel>>::link_one(id, &response.resource.id));
        Ok(response)
    }
}

pub trait Append<Rel: Relation>: AppendLinks<Rel> where Rel::Resource: RawUpdate + Deserialize {
    fn append(id: &Self::Id, post: Rel::Resource, rels: <Rel::Resource as RawUpdate>::Relationships) -> Result<PostResponse<Rel::Resource>>;
}

impl<T, Rel> Append<Rel> for T
where T:                AppendLinks<Rel>,
      Rel:              Relation,
      Rel::Resource:    RawPost {
    fn append(id: &Self::Id, post: Rel::Resource, rels: <Rel::Resource as RawUpdate>::Relationships) -> Result<PostResponse<Rel::Resource>> {
        let response = try!(post.post(rels));
        try!(<T as AppendLinks<Rel>>::append_links(id, &[response.resource.id.clone()]));
        Ok(response)
    }
}
