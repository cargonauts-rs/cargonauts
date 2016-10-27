use itertools::Itertools;

use api::{Resource, Error, Entity};
use api::raw::{Include, RawFetch, ResourceObject};
use router::IncludeQuery;
use _internal::_FetchRels;

pub trait Index: Resource {
    fn index() -> Result<Vec<Self>, Error>;
}

pub trait RawIndex: RawFetch {
    fn index(includes: &[IncludeQuery]) -> Result<IndexResponse<Self>, Error>;
}

impl<T> RawIndex for T where T: Index + _FetchRels {
    fn index(includes: &[IncludeQuery]) -> Result<IndexResponse<Self>, Error> {
        let mut resources = vec![];
        let mut include_objects = vec![];
        for resource in <T as Index>::index()? {
            let entity = Entity::Resource(resource);
            let (rels, incls) = <T as _FetchRels>::rels(&entity, includes)?;
            let resource = match entity {
                Entity::Resource(resource)  => resource,
                _                           => unreachable!()
            };
            include_objects.extend(incls);
            resources.push(ResourceObject {
                id: resource.id(),
                attributes: resource,
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
