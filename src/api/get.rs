use itertools::Itertools;

use api::{Resource, Error, Entity};
use api::raw::{Include, RawFetch, ResourceObject};
use router::IncludeQuery;
use _internal::_FetchRels;

pub trait Get: Resource {
    fn get(id: &Self::Id) -> Result<Self, Error>;
}

pub trait RawGet<I>: RawFetch {
    fn get(id: Entity<Self>, includes: &[IncludeQuery]) -> Result<GetResponse<I, Self>, Error>;
}

impl<I, T> RawGet<I> for T where T: Get + _FetchRels<I> {
    fn get(entity: Entity<Self>, includes: &[IncludeQuery]) -> Result<GetResponse<I, T>, Error> {
        let (id, entity) = match entity {
            Entity::Id(id)              => {
                let resource = <T as Get>::get(&id)?;
                (id, Entity::Resource(resource))
            }
            Entity::Resource(resource)  => {
                (resource.id(), Entity::Resource(resource))
            }
        };
        let (rels, includes) = <T as _FetchRels<I>>::rels(&entity, &includes)?;
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

pub struct GetResponse<I, T: RawFetch> {
    pub resource: ResourceObject<T>,
    pub includes: Vec<Include<I>>,
}
