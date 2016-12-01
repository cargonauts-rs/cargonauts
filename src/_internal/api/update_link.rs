use api::{Resource, Error, Entity};
use api::rel::{ToOne, HasOne, UpdateLink};
use presenter::Presenter;
use receiver::Receiver;
use router::{Router, ResourceRoute, Method, Request};
use Future;
use IntoFuture;

pub trait _MaybeUpdateLink<Rel: ToOne, P, C, R: Router>: Resource {
    fn attach(_: &mut R) { }
}

impl<T, Rel, P, C, R> _MaybeUpdateLink<Rel, P, C, R> for T
where
    T: HasOne<Rel>,
    Rel: ToOne,
    R: Router,
{ }

impl<T, Rel, P, C, R> _MaybeUpdateLink<Rel, P, C, R> for T
where
    T: UpdateLink<Rel>,
    Rel: ToOne,
    C: Receiver<(), R::Request>,
    P: Presenter<(), R>,
    R: Router,
{
    fn attach(router: &mut R) {
        _attach_update_link::<T, Rel, P, C, R>(router);
    }
}

pub fn _attach_update_link<T, Rel, P, C, R>(router: &mut R)
where
    T: UpdateLink<Rel>,
    Rel: ToOne,
    P: Presenter<(), R> + Presenter<(), R>,
    C: Receiver<(), R::Request>,
    R: Router,
{
    super::attach::<R, T>(router, ResourceRoute {
        method: Method::Patch,
        relation: Some((Rel::to_one(), true))
    }, _update_link::<R, T, Rel, P, C>);
}

fn _update_link<R, T, Rel, P, C>(request: R::Request, link_maker: R::LinkMaker) -> Box<Future<Item = R::Response, Error = ()>>
where
    T: UpdateLink<Rel>,
    Rel: ToOne,
    P: Presenter<(), R>,
    C: Receiver<(), R::Request>,
    R: Router,
{
    let options = request.resource_options();
    let presenter = <P as Presenter<(), R>>::prepare(options.field_set, link_maker);
    let id = match request.id() {
        Some(id)    => match id.parse() {
            Ok(id)  => id,
            Err(_)  => return Box::new(Ok(<P as Presenter<(), R>>::present_err(presenter, Error::Conflict)).into_future()),
        },
        None => return Box::new(Ok(<P as Presenter<(), R>>::present_err(presenter, Error::Conflict)).into_future()),
    };
    let identifier = match C::receive_to_one::<Rel>(request) {
        Ok(rel) => rel,
        Err(_)  => return Box::new(Ok(<P as Presenter<(), R>>::present_err(presenter, Error::Conflict)).into_future()),
    };
    let rel_id = match identifier {
        Some(identifier)    => {
            Some(try_status!(identifier.id.parse(), presenter))
        }
        None                => None
    };
    presenter.try_present(T::update_link(&Entity::Id(id), rel_id.as_ref()))
}
