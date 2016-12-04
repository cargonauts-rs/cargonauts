use api::{Resource, Error, Entity};
use api::raw::{Synchronous, RawHasPatch};
use api::rel::ToOne;
use api::rel::raw::PatchOne;
use receiver::PatchReceiver;
use router::{Router, Component, Method, Request};
use presenter::Presenter;
use Future;
use IntoFuture;

pub trait _MaybeAttachPatchOne<Rel: ToOne, P, C, R: Router>: Resource {
    fn attach(_: &mut R) { }
}

impl<T: Resource, Rel: ToOne, C, P, R: Router> _MaybeAttachPatchOne<Rel, P, C, R> for T { }

impl<T, Rel, C, P, R> _MaybeAttachPatchOne<Rel, P, C, R> for T
where
    T: PatchOne<P::Include, Rel>,
    Rel: ToOne,
    Rel::Resource: RawHasPatch<Synchronous>,
    P: Presenter<Rel::Resource, R>,
    C: PatchReceiver<Rel::Resource, R::Request, Synchronous>,
    R: Router,
{
    fn attach(router: &mut R) {
        _attach_patch_one::<R, T, Rel, P, C>(router);
    }
}

pub fn _attach_patch_one<R, T, Rel, P, C>(router: &mut R)
where
    R: Router,
    T: PatchOne<P::Include, Rel>,
    Rel: ToOne,
    Rel::Resource: RawHasPatch<Synchronous>,
    P: Presenter<Rel::Resource, R>,
    C: PatchReceiver<Rel::Resource, R::Request, Synchronous>,
{
    super::attach::<R, T>(
        router,
        Method::Update,
        Component::Related(Rel::to_one()),
        patch_one::<R, T, Rel, P, C>
    );
}

fn patch_one<R, T, Rel, P, C>(request: R::Request, link_maker: R::LinkMaker) -> Box<Future<Item = R::Response, Error = ()>>
where
    R: Router,
    T: PatchOne<P::Include, Rel>,
    Rel: ToOne,
    Rel::Resource: RawHasPatch<Synchronous>,
    P: Presenter<Rel::Resource, R>,
    C: PatchReceiver<Rel::Resource, R::Request, Synchronous>,
{
    let options = request.resource_options();
    let presenter = P::prepare(options.field_set, link_maker);
    let id = match request.id() {
        Some(id)    => try_status!(id.parse(), presenter),
        None        => try_status!(Err(()), presenter),
    };
    let received = try_status!(C::receive_patch(request), presenter);
    presenter.try_present(T::patch_one(Entity::Id(id), received))
}
