use api::{Delete, Resource, Error};
use router::{Router, Request, ResourceRoute, Method};
use presenter::Presenter;
use IntoFuture;
use Future;

pub trait _MaybeDelete<P, R: Router>: Resource {
    fn attach(_: &mut R) { }
}

impl<T: Resource, P, R: Router> _MaybeDelete<P, R> for T { }

impl<T, P, R> _MaybeDelete<P, R> for T
where
    T: Delete,
    P: Presenter<(), R>,
    R: Router,
{
    fn attach(router: &mut R) {
        _attach_delete::<R, T, P>(router);
    }
}


pub fn _attach_delete<R, T, P>(router: &mut R)
where
    T: Delete,
    P: Presenter<(), R>,
    R: Router,
{
    super::attach::<R, T>(router, ResourceRoute {
        method: Method::Delete,
        relation: None,
    }, delete::<R, T, P>);
}

fn delete<R, T, P>(request: R::Request, link_maker: R::LinkMaker) -> Box<Future<Item = R::Response, Error = ()>>
where
    T: Delete,
    P: Presenter<(), R>,
    R: Router,
{
    let presenter = P::prepare(None, link_maker);
    let id = match request.id() {
        Some(id)    => try_status!(id.parse(), presenter),
        None        => try_status!(Err(()), presenter),
    };
    presenter.try_present(T::delete(&id))
}
