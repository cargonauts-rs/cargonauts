use itertools::Itertools;

use api::{Resource, Error};
use api::raw::{Include, RawFetch, ResourceObject};
use _internal::_FetchRels;

pub trait Get: Resource {
    fn get(id: &Self::Id) -> Result<Self, Error>;
}

pub trait RawGet: RawFetch {
    fn get(id: Self::Id, includes: &[String]) -> Result<GetResponse<Self>, Error>;
}

impl<T> RawGet for T where T: Get + _FetchRels {
    fn get(id: Self::Id, includes: &[String]) -> Result<GetResponse<T>, Error> {
        let attributes = try!(<T as Get>::get(&id));
        let (rels, includes) = try!(<T as _FetchRels>::rels(&id, &includes));
        let includes = includes.into_iter()
            .unique_by(|include| (include.id.clone(), include.resource))
            .collect();
        Ok(GetResponse {
            resource: ResourceObject {
                id: id,
                attributes: attributes,
                relationships: rels,
            },
            includes: includes,
        })
    }
}

pub struct GetResponse<T: RawFetch> {
    pub resource: ResourceObject<T>,
    pub includes: Vec<Include>,
}
