use api::{Remove, Resource, Error};
use api::raw::RawResource;
use router::{Router, Component, Method};
use presenter::Presenter;
use receiver::Receiver;
use IntoFuture;
use Future;

pub trait _MaybeAttachRemove<P, C, R: Router>: Resource {
    fn attach(_: &mut R) { }
}

impl<T: Resource, P, C, R: Router> _MaybeAttachRemove<P, C, R> for T { }

impl<T, P, C, R> _MaybeAttachRemove<P, C, R> for T
where
    T: Remove + RawResource,
    P: Presenter<(), R>,
    C: Receiver<T, R::Request>,
    R: Router,
{
    fn attach(router: &mut R) {
        _attach_remove::<R, T, P, C>(router);
    }
}

pub fn _attach_remove<R, T, P, C>(router: &mut R)
where
    R: Router,
    T: Remove + RawResource,
    P: Presenter<(), R>,
    C: Receiver<T, R::Request>,
{
    super::attach::<R, T>(
        router,
        Method::Destroy,
        Component::Collection,
        remove::<R, T, P, C>
    );
}

fn remove<R, T, P, C>(request: R::Request, link_maker: R::LinkMaker) -> Box<Future<Item = R::Response, Error = ()>>
where
    R: Router,
    T: Remove + RawResource,
    P: Presenter<(), R>,
    C: Receiver<T, R::Request>,
{
    let presenter = P::prepare(None, link_maker);
    let identifiers = try_status!(C::receive_identifiers(request), presenter);
    let ids = try_status!(identifiers.into_iter().map(|identifier| identifier.id.parse()).collect::<Result<Vec<_>, _>>(), presenter);
    presenter.try_present(T::remove(&ids))
}
