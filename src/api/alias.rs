use api::{Resource, Result, Entity};
use api::raw::{RawFetch, RawGet, GetResponse};
use router::IncludeQuery;

pub struct AliasRequest {
    pub alias: &'static str,
}

pub trait GetAliased: Resource {
    fn get(request: AliasRequest) -> Result<Entity<Self>>;
}

pub trait RawGetAliased<I>: RawFetch {
    fn get(request: AliasRequest, includes: &[IncludeQuery]) -> Result<GetResponse<I, Self>>;
}

impl<I, T> RawGetAliased<I> for T where T: GetAliased + RawGet<I> {
    fn get(request: AliasRequest, includes: &[IncludeQuery]) -> Result<GetResponse<I, Self>> {
        let entity = <T as GetAliased>::get(request)?;
        <T as RawGet<I>>::get(entity, includes)
    }
}
