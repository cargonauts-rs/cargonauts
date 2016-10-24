use itertools::Itertools;

use api::raw::{Include, RawFetch, ResourceObject};
use api::{Resource, Error};
use _internal::_FetchRels;

pub trait Index: Resource {
    fn index() -> Result<Vec<Self>, Error>;
}

pub trait RawIndex: RawFetch {
    fn index(includes: &[String]) -> Result<IndexResponse<Self>, Error>;
}

impl<T> RawIndex for T where T: Index + _FetchRels {
    fn index(includes: &[String]) -> Result<IndexResponse<Self>, Error> {
        let mut resources = vec![];
        let mut include_objects = vec![];
        for attributes in try!(<T as Index>::index()) {
            let id = attributes.id();
            let (rels, incls) = try!(<T as _FetchRels>::rels(&id, includes));
            include_objects.extend(incls);
            resources.push(ResourceObject {
                id: id,
                attributes: attributes,
                relationships: rels,
            });
        }
        let includes = include_objects.into_iter()
            .unique_by(|include| (include.id.clone(), include.resource))
            .collect();
        Ok(IndexResponse {
            resources: resources,
            includes: includes,
        })
    }
}

pub struct IndexResponse<T: RawFetch> {
    pub resources: Vec<ResourceObject<T>>,
    pub includes: Vec<Include>,
}
