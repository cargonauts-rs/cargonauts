use api::Resource;
use router::{Router, Component, Method, Request};
use presenter::Presenter;
use api::raw::RawIndex;
use Future;

fn index<R, T, P>(request: R::Request, link_maker: R::LinkMaker) -> Box<Future<Item = R::Response, Error = ()>>
where 
    R: Router,
    T: RawIndex<P::Include>,
    P: Presenter<T, R>,
{
    let options = request.collection_options();
    let presenter = P::prepare(options.field_set, link_maker);
    presenter.try_present(T::index(options.includes, options.sort, options.page))
}

pub fn _attach_index<R, T, P>(router: &mut R)
where 
    R: Router,
    T: RawIndex<P::Include>,
    P: Presenter<T, R>,
{
    super::attach::<R, T>(
        router,
        Method::Read,
        Component::Collection,
        index::<R, T, P>
    );
}

pub trait _MaybeIndex<P, R: Router>: Resource {
    fn attach(_: &mut R) { }
}

impl<T: Resource, P, R: Router> _MaybeIndex<P, R> for T { }

impl<T, P, R> _MaybeIndex<P, R> for T
where
    T: RawIndex<P::Include>,
    P: Presenter<T, R>,
    R: Router,
{
    fn attach(router: &mut R) {
        _attach_index::<R, T, P>(router);
    }
}

