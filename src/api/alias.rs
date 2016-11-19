use api::{Get, Index, Error};
use api::index::raw_index;
use api::get::raw_get;
use api::raw::{RawFetch, ResourceResponse, CollectionResponse};
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

pub trait RawGetAliased<I>: RawFetch {
    type RawGetAliasedFut: IntoFuture<Item = ResourceResponse<I, Self>, Error = Error>;
    fn get(request: AliasRequest, includes: &[IncludeQuery]) -> Self::RawGetAliasedFut;
}

impl<I, T> RawGetAliased<I> for T where T: GetAliased + _FetchRels<I> {
    type RawGetAliasedFut = Result<ResourceResponse<I, Self>, Error>;
    fn get(request: AliasRequest, includes: &[IncludeQuery]) -> Self::RawGetAliasedFut {
        let resource = <T as GetAliased>::get(request).into_future().wait()?;
        raw_get(resource.id(), resource, includes)
    }
}

pub trait IndexAliased: Index {
    type IndexAliasedFut: IntoFuture<Item = Vec<Self>, Error = Error>;
    fn index(request: AliasRequest) -> Self::IndexAliasedFut;
}

pub trait RawIndexAliased<I>: RawFetch {
    type RawIndexAliasedFut: IntoFuture<Item = CollectionResponse<I, Self>, Error = Error>;
    fn index(request: AliasRequest, includes: &[IncludeQuery], sorts: &[SortQuery]) -> Self::RawIndexAliasedFut;
}

impl<I, T> RawIndexAliased<I> for T where T: IndexAliased + _FetchRels<I> {
    type RawIndexAliasedFut = Result<CollectionResponse<I, Self>, Error>;
    fn index(request: AliasRequest, includes: &[IncludeQuery], sorts: &[SortQuery]) -> Self::RawIndexAliasedFut {
        let collection = <T as IndexAliased>::index(request).into_future().wait()?;
        raw_index(collection, includes, sorts)
    }
}
