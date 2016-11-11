use itertools::Itertools;

use api::sort::MaybeSort;
use api::{Resource, Error, Entity};
use api::raw::{Include, RawFetch, ResourceObject};
use router::{IncludeQuery, SortQuery, Pagination};
use _internal::_FetchRels;
use IntoFuture;
use futures::Future;

pub trait Index: Resource {
    type IndexFut: IntoFuture<Item = Vec<Self>, Error = Error>;
    fn index() -> Self::IndexFut;
}

pub trait Paginated: Index {
    type PaginatedFut: IntoFuture<Item = Vec<Self>, Error = Error>;
    fn paginated_index(pagination: &Pagination) -> Self::PaginatedFut;
}

pub trait RawIndex<I>: RawFetch {
    type RawIndexFut: IntoFuture<Item = IndexResponse<I, Self>, Error = Error>;
    fn index(includes: &[IncludeQuery], sorts: &[SortQuery], pagination: &Option<Pagination>) -> Self::RawIndexFut;
}

impl<I, T> RawIndex<I> for T where T: Index + _FetchRels<I> {
    type RawIndexFut = Result<IndexResponse<I, Self>, Error>;
    default fn index(includes: &[IncludeQuery], sorts: &[SortQuery], pagination: &Option<Pagination>) -> Self::RawIndexFut {
        match pagination.is_none() {
            true    => raw_index(<T as Index>::index().into_future().wait()?, includes, sorts),
            false   => Err(Error::BadRequest),
        }
    }
}
impl<I, T> RawIndex<I> for T where T: Paginated + _FetchRels<I> {
    fn index(includes: &[IncludeQuery], sorts: &[SortQuery], pagination: &Option<Pagination>) -> Self::RawIndexFut {
        let index = match pagination.as_ref() {
            Some(pagination)    => <T as Paginated>::paginated_index(pagination).into_future().wait()?,
            None                => <T as Index>::index().into_future().wait()?,
        };
        raw_index(index, includes, sorts)
    }
}

pub struct IndexResponse<I, T: RawFetch> {
    pub resources: Vec<ResourceObject<T>>,
    pub includes: Vec<Include<I>>,
}

fn raw_index<I, T: _FetchRels<I>>(index: Vec<T>, includes: &[IncludeQuery], sorts: &[SortQuery]) -> Result<IndexResponse<I, T>, Error> {
    let mut resources = vec![];
    let mut include_objects = vec![];
    for resource in index {
        let entity = Entity::Resource(resource);
        let (rels, incls) = <T as _FetchRels<I>>::rels(&entity, includes)?;
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
