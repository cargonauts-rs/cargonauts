use api::{Resource, Error, Entity};
use api::rel::ToOne;
use api::rel::raw::DeleteOne;
use router::{Router, Component, Method, Request};
use presenter::Presenter;
use Future;
use IntoFuture;

pub trait _MaybeDeleteOne<Rel: ToOne, P, R: Router>: Resource {
    fn attach(_: &mut R) { }
}

impl<T: Resource, Rel: ToOne, P, R: Router> _MaybeDeleteOne<Rel, P, R> for T { }

impl<T, Rel, P, R> _MaybeDeleteOne<Rel, P, R> for T
where
    T: DeleteOne<Rel>,
    Rel: ToOne,
    P: Presenter<(), R>,
    R: Router,
{
    fn attach(router: &mut R) {
        _attach_delete_one::<R, T, Rel, P>(router);
    }
}

pub fn _attach_delete_one<R, T, Rel, P>(router: &mut R)
where
    R: Router,
    T: DeleteOne<Rel>,
    Rel: ToOne,
    P: Presenter<(), R>,
{
    super::attach::<R, T>(
        router,
        Method::Destroy,
        Component::Related(Rel::to_one()),
        delete_one::<R, T, Rel, P>
    );
}

fn delete_one<R, T, Rel, P>(request: R::Request, link_maker: R::LinkMaker) -> Box<Future<Item = R::Response, Error = ()>>
where
    T: DeleteOne<Rel>,
    Rel: ToOne,
    P: Presenter<(), R>,
    R: Router,
{
    let presenter = P::prepare(None, link_maker);
    let id = match request.id() {
        Some(id)    => try_status!(id.parse(), presenter),
        None        => try_status!(Err(()), presenter),
    };
    presenter.try_present(T::delete_one(Entity::Id(id)))
}
