use api::raw::{RawGetAliased, RawIndexAliased};
use router::{Request, Router, Method};
use futures::Future;
use presenter::Presenter;

pub fn _attach_get_aliased<R, T, P>(router: &mut R, route: &'static str)
where
    R: Router,
    T: RawGetAliased<P::Include>,
    P: Presenter<T, R>,
{ router.attach_alias(route, Method::Read, get_aliased::<R, T, P>); }

pub fn _attach_index_aliased<R, T, P>(router: &mut R, route: &'static str)
where
    R: Router,
    T: RawIndexAliased<P::Include>,
    P: Presenter<T, R>,
{ router.attach_alias(route, Method::Read, index_aliased::<R, T, P>); }

fn get_aliased<R, T, P>(request: R::Request, link_maker: R::LinkMaker) -> Box<Future<Item = R::Response, Error = ()>>
where
    R: Router,
    T: RawGetAliased<P::Include>,
    P: Presenter<T, R>,
{
    let options = request.resource_options();
    let presenter = P::prepare(options.field_set, link_maker);
    presenter.try_present(T::get(request.alias_info(), options.includes))
}

fn index_aliased<R, T, P>(request: R::Request, link_maker: R::LinkMaker) -> Box<Future<Item = R::Response, Error = ()>>
where
    R: Router,
    T: RawIndexAliased<P::Include>,
    P: Presenter<T, R>,
{
    let options = request.collection_options();
    let presenter = P::prepare(options.field_set, link_maker);
    presenter.try_present(T::index(request.alias_info(), options.includes, options.sort))
}
