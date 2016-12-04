use api::{Resource, Error, Entity};
use api::raw::RawResource;
use api::rel::ToMany;
use api::rel::raw::IndexMany;
use router::{Router, Component, Method, Request};
use presenter::Presenter;
use Future;
use IntoFuture;

pub trait _MaybeAttachIndexMany<Rel: ToMany, P, R: Router>: Resource {
    fn attach(_: &mut R) { }
}

impl<T: Resource, Rel: ToMany, P, R: Router> _MaybeAttachIndexMany<Rel, P, R> for T { }

impl<T, Rel, P, R> _MaybeAttachIndexMany<Rel, P, R> for T
where
    T: IndexMany<P::Include, Rel>,
    Rel: ToMany,
    Rel::Resource: RawResource,
    P: Presenter<Rel::Resource, R>,
    R: Router,
{
    fn attach(router: &mut R) {
        _attach_index_many::<R, T, Rel, P>(router);
    }
}
pub fn _attach_index_many<R, T, Rel, P>(router: &mut R)
where
    T: IndexMany<P::Include, Rel>,
    Rel: ToMany,
    Rel::Resource: RawResource,
    P: Presenter<Rel::Resource, R>,
    R: Router,
{
    super::attach::<R, T>(
        router,
        Method::Read,
        Component::Related(Rel::to_many()),
        index_many::<R, T, Rel, P>
    );
}

fn index_many<R, T, Rel, P>(request: R::Request, link_maker: R::LinkMaker) -> Box<Future<Item = R::Response, Error = ()>>
where
    T: IndexMany<P::Include, Rel>,
    Rel: ToMany,
    Rel::Resource: RawResource,
    P: Presenter<Rel::Resource, R>,
    R: Router,
{
    let options = request.collection_options();
    let presenter = P::prepare(options.field_set, link_maker);
    let id = match request.id() {
        Some(id)    => try_status!(id.parse(), presenter),
        None        => try_status!(Err(()), presenter),
    };
    presenter.try_present(T::index_many(Entity::Id(id), options.includes))
}
