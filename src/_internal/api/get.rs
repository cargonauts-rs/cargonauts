use api::{Resource, Error};
use router::{Router, Component, Method, Request};
use presenter::Presenter;
use api::raw::RawGet;
use Future;
use IntoFuture;

pub trait _MaybeAttachGet<P, R: Router>: Resource {
    fn attach(_: &mut R) { }
}

impl<T: Resource, P, R: Router> _MaybeAttachGet<P, R> for T { }

impl<T, P, R> _MaybeAttachGet<P, R> for T
where
    T: RawGet<P::Include>,
    P: Presenter<T, R>,
    R: Router,
{
    fn attach(router: &mut R) {
        _attach_get::<R, T, P>(router)
    }
}

pub fn _attach_get<R, T, P>(router: &mut R)
where
    T: RawGet<P::Include>,
    P: Presenter<T, R>,
    R: Router,
{
    super::attach::<R, T>(
        router,
        Method::Read,
        Component::Resource,
        get::<R, T, P>
    );
}

fn get<R, T, P>(request: R::Request, link_maker: R::LinkMaker) -> Box<Future<Item = R::Response, Error = ()>>
where
    T: RawGet<P::Include>,
    P: Presenter<T, R>,
    R: Router,
{
    let options = request.resource_options();
    let presenter = P::prepare(options.field_set, link_maker);
    let id = match request.id() {
        Some(id)    => try_status!(id.parse(), presenter),
        None        => try_status!(Err(()), presenter),
    };
    presenter.try_present(T::get(id, options.includes))
}
