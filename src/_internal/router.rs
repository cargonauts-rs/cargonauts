#![allow(unused_parens)]

use api::raw;
use router::{Request, Router, Method};
use futures::Future;
use presenter::Presenter;

macro_rules! try_status {
    ($x:expr, $p:expr)  => {
        match $x {
            Ok(x)   => x,
            Err(_)  => {
                return Box::new(Ok($p.present_err(Error::Conflict)).into_future())
            }
        }
    };
}

pub struct _Router<'a, R: Router + 'a> {
    pub router: &'a mut R,
}

impl<'a, R: Router> _Router<'a, R> {
    pub fn new(router: &'a mut R) -> _Router<'a, R> {
        _Router {
            router: router,
        }
    }

    pub fn attach_get_alias<T, P>(&mut self, route: &'static str)
    where
        T: raw::RawGetAliased<P::Include>,
        P: Presenter<T, R>,
    { 
        fn get_aliased<R, T, P>(request: R::Request, link_maker: R::LinkMaker) -> Box<Future<Item = R::Response, Error = ()>>
        where
            T: raw::RawGetAliased<P::Include>,
            P: Presenter<T, R>,
            R: Router,
        {
            let options = request.resource_options();
            let presenter = P::prepare(options.field_set, link_maker);
            presenter.try_present(T::get(request.alias_info(), options.includes))
        }
        self.router.attach_alias(route, Method::Get, get_aliased::<R, T, P>);
    }

    pub fn attach_index_aliased<T, P>(&mut self, route: &'static str)
    where
        T: raw::RawIndexAliased<P::Include>,
        P: Presenter<T, R>,
    {
        fn index_aliased<R, T, P>(request: R::Request, link_maker: R::LinkMaker) -> Box<Future<Item = R::Response, Error = ()>>
        where
            T: raw::RawIndexAliased<P::Include>,
            P: Presenter<T, R>,
            R: Router,
        {
            let options = request.collection_options();
            let presenter = P::prepare(options.field_set, link_maker);
            presenter.try_present(T::index(request.alias_info(), options.includes, options.sort))
        }
        self.router.attach_alias(route, Method::Index, index_aliased::<R, T, P>);
    }
}
