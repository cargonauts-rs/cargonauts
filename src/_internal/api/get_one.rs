use api::{Resource, Error, Entity};
use api::raw::RawResource;
use api::rel::ToOne;
use api::rel::raw::GetOne;
use router::{Router, Component, Method, Request};
use presenter::Presenter;
use Future;
use IntoFuture;

pub trait _MaybeGetOne<Rel: ToOne, P, R: Router>: Resource {
    fn attach(_: &mut R) { }
}

impl<T: Resource, Rel: ToOne, P, R: Router> _MaybeGetOne<Rel, P, R> for T { }

impl<T, Rel, P, R> _MaybeGetOne<Rel, P, R> for T
where
    T: GetOne<P::Include, Rel>,
    Rel: ToOne,
    Rel::Resource: RawResource,
    P: Presenter<Rel::Resource, R>,
    R: Router,
{
    fn attach(router: &mut R) {
        _attach_get_one::<R, T, Rel, P>(router);
    }
}

pub fn _attach_get_one<R, T, Rel, P>(router: &mut R)
where
    T: GetOne<P::Include, Rel>,
    Rel: ToOne,
    Rel::Resource: RawResource,
    P: Presenter<Rel::Resource, R>,
    R: Router,
{
    super::attach::<R, T>(
        router,
        Method::Read,
        Component::Related(Rel::to_one()),
        get_one::<R, T, Rel, P>
    );
}

fn get_one<R, T, Rel, P>(request: R::Request, link_maker: R::LinkMaker) -> Box<Future<Item = R::Response, Error = ()>>
where
    T: GetOne<P::Include, Rel>,
    Rel: ToOne,
    Rel::Resource: RawResource,
    P: Presenter<Rel::Resource, R>,
    R: Router,
{
    let options = request.resource_options();
    let presenter = P::prepare(options.field_set, link_maker);
    let id = match request.id() {
        Some(id)    => try_status!(id.parse(), presenter),
        None        => try_status!(Err(()), presenter),
    };
    presenter.try_present(T::get_one(Entity::Id(id), options.includes))
}
