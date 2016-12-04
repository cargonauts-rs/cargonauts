use api::{Resource, Error};
use api::raw::RawReplace;
use presenter::Presenter;
use receiver::Receiver;
use router::{Request, Router, Component, Method};
use IntoFuture;
use Future;

pub trait _MaybeReplace<P, C, R: Router>: Resource {
    fn attach(_: &mut R) { }
}

impl<T: Resource, P, C, R: Router> _MaybeReplace<P, C, R> for T { }

impl<T, P, C, R> _MaybeReplace<P, C, R> for T
where
    T: RawReplace<P::Include>,
    P: Presenter<T, R>,
    C: Receiver<T, R::Request>,
    R: Router,
{
    fn attach(router: &mut R) {
        _attach_replace::<R, T, P, C>(router);
    }
}

pub fn _attach_replace<R, T, P, C>(router: &mut R)
where
    R: Router,
    T: RawReplace<P::Include>,
    P: Presenter<T, R>,
    C: Receiver<T, R::Request>,
{
    super::attach::<R, T>(
        router,
        Method::Update,
        Component::Collection,
        replace::<R, T, P, C>
    );
}

fn replace<R, T, P, C>(request: R::Request, link_maker: R::LinkMaker) -> Box<Future<Item = R::Response, Error = ()>>
where
    T: RawReplace<P::Include>,
    P: Presenter<T, R>,
    R: Router,
    C: Receiver<T, R::Request>,
{
    let options = request.collection_options();
    let presenter = P::prepare(options.field_set, link_maker);
    let received = try_status!(C::receive_collection(request), presenter);
    presenter.try_present(T::replace(received))
}
