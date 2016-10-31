use itertools::Itertools;

use api::{Resource, Error, Entity};
use api::raw::{Include, RawFetch, ResourceObject};
use router::IncludeQuery;
use _internal::_FetchRels;
use repr::Presenter;

pub trait Get: Resource {
    fn get(id: &Self::Id) -> Result<Self, Error>;
}

pub trait RawGet: RawFetch {
    fn get<P: Presenter>(id: Self::Id, includes: &[IncludeQuery]) -> Result<GetResponse<P, Self>, Error>;
}

impl<T> RawGet for T where T: Get + _FetchRels {
    fn get<P: Presenter>(id: Self::Id, includes: &[IncludeQuery]) -> Result<GetResponse<P, T>, Error> {
        let entity = Entity::Resource(<T as Get>::get(&id)?);
        let (rels, includes) = <T as _FetchRels>::rels(&entity, &includes)?;
        let includes = includes.into_iter()
            .unique_by(|include| (include.id.clone(), include.resource))
            .collect();
        let resource = match entity {
            Entity::Resource(resource)  => resource,
            _                           => unreachable!()
        };
        Ok(GetResponse {
            resource: ResourceObject {
                id: id,
                attributes: resource,
                relationships: rels,
            },
            includes: includes,
        })
    }
}

pub struct GetResponse<P: Presenter, T: RawFetch> {
    pub resource: ResourceObject<T>,
    pub includes: Vec<Include<P>>,
}
