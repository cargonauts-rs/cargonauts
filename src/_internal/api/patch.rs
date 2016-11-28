use api::{Resource, Error};
use api::async::raw::{RawPatchAsync, Asynchronous};
use api::raw::{RawPatch, Synchronous};
use presenter::Presenter;
use receiver::PatchReceiver;
use router::{Router, ResourceRoute, Method, Request};
use Future;
use IntoFuture;

pub trait _MaybePatch<P, C, R: Router>: Resource {
    fn attach(_: &mut R) { }
}

impl<T: Resource, P, C, R: Router> _MaybePatch<P, C, R> for T { }

impl<T, P, C, R> _MaybePatch<P, C, R> for T
where
    T: RawPatch<P::Include>,
    P: Presenter<T, R>,
    C: PatchReceiver<T, R::Request, Synchronous>,
    R: Router,
{
    fn attach(router: &mut R) {
        _attach_patch::<R, T, P, C>(router);
    }
}

pub trait _MaybePatchAsync<P, C, R: Router>: Resource {
    fn attach(_: &mut R) { }
}

impl<T: Resource, P, C, R: Router> _MaybePatchAsync<P, C, R> for T { }

impl<T, P, C, R> _MaybePatchAsync<P, C, R> for T
where
    T: RawPatchAsync,
    P: Presenter<T::Job, R>,
    C: PatchReceiver<T, R::Request, Asynchronous>,
    R: Router,
{
    fn attach(router: &mut R) {
        _attach_patch_async::<R, T, P, C>(router);
    }
}

pub fn _attach_patch<R, T, P, C>(router: &mut R)
where
    R: Router,
    T: RawPatch<P::Include>,
    P: Presenter<T, R>,
    C: PatchReceiver<T, R::Request, Synchronous>,
{
    super::attach::<R, T>(router, ResourceRoute {
        method: Method::Patch,
        relation: None,
    }, patch::<R, T, P, C>);
}

pub fn _attach_patch_async<R, T, P, C>(router: &mut R)
where
    R: Router,
    T: RawPatchAsync,
    P: Presenter<T::Job, R>,
    C: PatchReceiver<T, R::Request, Asynchronous>,
{
    super::attach::<R, T>(router, ResourceRoute {
        method: Method::Patch,
        relation: None,
    }, patch_async::<R, T, P, C>);
}

fn patch<R, T, P, C>(request: R::Request, link_maker: R::LinkMaker) -> Box<Future<Item = R::Response, Error = ()>>
where
    R: Router,
    T: RawPatch<P::Include>,
    P: Presenter<T, R>,
    C: PatchReceiver<T, R::Request, Synchronous>,
{
    let options = request.resource_options();
    let presenter = P::prepare(options.field_set, link_maker);
    let id = match request.id() {
        Some(id)    => try_status!(id.parse(), presenter),
        None        => try_status!(Err(()), presenter),
    };
    let received = try_status!(C::receive_patch(request), presenter);
    presenter.try_present(T::patch(id, received))
}

fn patch_async<R, T, P, C>(request: R::Request, link_maker: R::LinkMaker) -> Box<Future<Item = R::Response, Error = ()>>
where
    T: RawPatchAsync,
    P: Presenter<T::Job, R>,
    R: Router,
    C: PatchReceiver<T, R::Request, Asynchronous>,
{
    let presenter = P::prepare(None, link_maker);
    let id = match request.id() {
        Some(id)    => try_status!(id.parse(), presenter),
        None        => try_status!(Err(()), presenter),
    };
    let received = try_status!(C::receive_patch(request), presenter);
    presenter.try_present(T::patch_async(id, received))
}
