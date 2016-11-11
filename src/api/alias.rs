use api::{Resource, Entity, Error};
use api::raw::{RawFetch, RawGet, GetResponse};
use router::IncludeQuery;
use futures::Future;
use IntoFuture;

pub struct AliasRequest {
    pub alias: &'static str,
}

pub trait GetAliased: Resource {
    type GetAliasedFut: IntoFuture<Item = Entity<Self>, Error = Error>;
    fn get(request: AliasRequest) -> Self::GetAliasedFut;
}

pub trait RawGetAliased<I>: RawFetch {
    type RawGetAliasedFut: IntoFuture<Item = GetResponse<I, Self>, Error = Error>;
    fn get(request: AliasRequest, includes: &[IncludeQuery]) -> Self::RawGetAliasedFut;
}

impl<I, T> RawGetAliased<I> for T where T: GetAliased + RawGet<I> {
    type RawGetAliasedFut = Result<GetResponse<I, Self>, Error>;
    fn get(request: AliasRequest, includes: &[IncludeQuery]) -> Self::RawGetAliasedFut {
        let entity = <T as GetAliased>::get(request).into_future().wait()?;
        match entity {
            Entity::Id(id)              => <T as RawGet<I>>::get_id(id, includes).into_future().wait(),
            Entity::Resource(resource)  => <T as RawGet<I>>::get_resource(resource, includes).into_future().wait(),
        }
    }
}
