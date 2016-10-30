use itertools::Itertools;

use api::sort::MaybeSort;
use api::{Resource, Error, Entity};
use api::raw::{Include, RawFetch, ResourceObject};
use router::{IncludeQuery, SortQuery, Pagination};
use _internal::_FetchRels;
use Serializer;

pub trait Index: Resource {
    fn index() -> Result<Vec<Self>, Error>;
}

pub trait Paginated: Index {
    fn paginated_index(pagination: &Pagination) -> Result<Vec<Self>, Error>;
}

pub trait RawIndex: RawFetch {
    fn index<S: Serializer>(includes: &[IncludeQuery], sorts: &[SortQuery], pagination: &Option<Pagination>) -> Result<IndexResponse<S, Self>, Error>;
}

impl<T> RawIndex for T where T: Index + _FetchRels {
    default fn index<S: Serializer>(includes: &[IncludeQuery], sorts: &[SortQuery], pagination: &Option<Pagination>) -> Result<IndexResponse<S, Self>, Error> {
        match pagination.is_none() {
            true    => raw_index(<T as Index>::index()?, includes, sorts),
            false   => Err(Error::BadRequest),
        }
    }
}
impl<T> RawIndex for T where T: Paginated + _FetchRels {
    fn index<S: Serializer>(includes: &[IncludeQuery], sorts: &[SortQuery], pagination: &Option<Pagination>) -> Result<IndexResponse<S, Self>, Error> {
        let index = match pagination.as_ref() {
            Some(pagination)    => <T as Paginated>::paginated_index(pagination)?,
            None                => <T as Index>::index()?,
        };
        raw_index(index, includes, sorts)
    }
}

pub struct IndexResponse<S: Serializer, T: RawFetch> {
    pub resources: Vec<ResourceObject<T>>,
    pub includes: Vec<Include<S>>,
}

fn raw_index<S: Serializer, T: _FetchRels>(index: Vec<T>, includes: &[IncludeQuery], sorts: &[SortQuery]) -> Result<IndexResponse<S, T>, Error> {
    let mut resources = vec![];
    let mut include_objects = vec![];
    for resource in index {
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
    for sort in sorts.iter().rev() {
        if <T as MaybeSort>::sorts_by(&sort.field) {
            resources.sort_by(|left, right| match sort.ascending {
                true    => <T as MaybeSort>::cmp(&sort.field, &left.attributes, &right.attributes),
                false   => <T as MaybeSort>::cmp(&sort.field, &left.attributes, &right.attributes).reverse(),
            });
        } else {
            return Err(Error::BadRequest)
        }
    }
    let includes = include_objects.into_iter()
        .unique_by(|include| (include.id.clone(), include.resource))
        .collect();
    Ok(IndexResponse {
        resources: resources,
        includes: includes,
    })
}
