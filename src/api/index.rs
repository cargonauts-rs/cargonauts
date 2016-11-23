use itertools::Itertools;

use api::sort::MaybeSort;
use api::{Resource, Error, Entity};
use api::raw::{CollectionResponse, RawFetch, ResourceObject};
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
    fn paginated_index(pagination: Pagination) -> Self::PaginatedFut;
}

pub trait RawIndex<I>: RawFetch {
    type RawIndexFut: Future<Item = CollectionResponse<I, Self>, Error = Error>;
    fn index(includes: Vec<IncludeQuery>, sorts: Vec<SortQuery>, pagination: Option<Pagination>) -> Self::RawIndexFut;
}

impl<I, T> RawIndex<I> for T where T: Index + _FetchRels<I>, I: 'static {
    type RawIndexFut = Box<Future<Item = CollectionResponse<I, Self>, Error = Error>>;
    default fn index(includes: Vec<IncludeQuery>, sorts: Vec<SortQuery>, pagination: Option<Pagination>) -> Self::RawIndexFut {
        match pagination.is_none() {
            true    => {
                Box::new(<T as Index>::index().into_future().and_then(|resources| {
                    raw_index(resources, includes, sorts)
                }))
            }
            false   => Box::new(Err(Error::BadRequest).into_future()),
        }
    }
}
impl<I, T> RawIndex<I> for T where T: Paginated + _FetchRels<I>, I: 'static {
    fn index(includes: Vec<IncludeQuery>, sorts: Vec<SortQuery>, pagination: Option<Pagination>) -> Self::RawIndexFut {
        match pagination {
            Some(pagination)    => Box::new(<T as Paginated>::paginated_index(pagination).into_future().and_then(|resources| {
                raw_index(resources, includes, sorts)
            })),
            None                => Box::new(<T as Index>::index().into_future().and_then(|resources| {
                raw_index(resources, includes, sorts)
            })),
        }
    }
}

pub fn raw_index<I: 'static, T: _FetchRels<I>>(index: Vec<T>, includes: Vec<IncludeQuery>, sorts: Vec<SortQuery>) -> Box<Future<Item = CollectionResponse<I, T>, Error = Error>> {
    let mut resources = vec![];
    let mut include_objects = vec![];
    for resource in index {
        let entity = Entity::Resource(resource);
        let (rels, incls) = match <T as _FetchRels<I>>::rels(&entity, &includes) {
            Ok(data)    => data,
            Err(err)    => return Box::new(Err(err).into_future()),
        };
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
            return Box::new(Err(Error::BadRequest).into_future())
        }
    }
    let includes = include_objects.into_iter()
        .unique_by(|include| (include.id.clone(), include.resource))
        .collect();
    Box::new(Ok(CollectionResponse {
        resources: resources,
        includes: includes,
    }).into_future())
}
