use api::{Get, Index, Error};
use api::index::raw_index;
use api::get::raw_get;
use api::raw::{RawResource, ResourceResponse, CollectionResponse};
use router::{IncludeQuery, SortQuery};
use _internal::_FetchRels;
use futures::Future;
use IntoFuture;

pub struct AliasRequest {
    pub alias: &'static str,
}

pub trait GetAliased: Get {
    type GetAliasedFut: IntoFuture<Item = Self, Error = Error>;
    fn get(request: AliasRequest) -> Self::GetAliasedFut;
}

pub trait RawGetAliased<I>: RawResource {
    type RawGetAliasedFut: IntoFuture<Item = ResourceResponse<I, Self>, Error = Error>;
    fn get(request: AliasRequest, includes: Vec<IncludeQuery>) -> Self::RawGetAliasedFut;
}

impl<I, T> RawGetAliased<I> for T where T: GetAliased + _FetchRels<I>, I: 'static {
    type RawGetAliasedFut = Box<Future<Item = ResourceResponse<I, Self>, Error = Error>>;
    fn get(request: AliasRequest, includes: Vec<IncludeQuery>) -> Self::RawGetAliasedFut {
        Box::new(<T as GetAliased>::get(request).into_future().and_then(move |resource| {
            raw_get(resource.id(), resource, includes)
        }))
    }
}

pub trait IndexAliased: Index {
    type IndexAliasedFut: IntoFuture<Item = Vec<Self>, Error = Error>;
    fn index(request: AliasRequest) -> Self::IndexAliasedFut;
}

pub trait RawIndexAliased<I>: RawResource {
    type RawIndexAliasedFut: IntoFuture<Item = CollectionResponse<I, Self>, Error = Error>;
    fn index(request: AliasRequest, includes: Vec<IncludeQuery>, sorts: Vec<SortQuery>) -> Self::RawIndexAliasedFut;
}

impl<I, T> RawIndexAliased<I> for T where T: IndexAliased + _FetchRels<I>, I: 'static {
    type RawIndexAliasedFut = Box<Future<Item = CollectionResponse<I, Self>, Error = Error>>;
    fn index(request: AliasRequest, includes: Vec<IncludeQuery>, sorts: Vec<SortQuery>) -> Self::RawIndexAliasedFut {
        Box::new(<T as IndexAliased>::index(request).into_future().and_then(|collection| {
            raw_index(collection, includes, sorts)
        }))
    }
}
