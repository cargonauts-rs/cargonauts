use api::{Resource, Error, Entity};
use api::raw::RawResource;
use api::rel::{ToMany, RemoveLinks};
use receiver::Receiver;
use router::{Router, ResourceRoute, Method, Request};
use presenter::Presenter;
use Future;
use IntoFuture;

pub trait _MaybeRemoveLinks<Rel: ToMany, P, C, R: Router>: Resource {
    fn attach(_: &mut R) { }
}

impl<T: Resource, Rel: ToMany, P, C, R: Router> _MaybeRemoveLinks<Rel, P, C, R> for T { }

impl<T, Rel, P, C, R> _MaybeRemoveLinks<Rel, P, C, R> for T
where
    T: RemoveLinks<Rel>,
    Rel: ToMany,
    Rel::Resource: RawResource,
    P: Presenter<(), R>,
    C: Receiver<Rel::Resource, R::Request>,
    R: Router,
{
    fn attach(router: &mut R) {
        _attach_remove_links::<R, T, Rel, P, C>(router);
    }
}

pub fn _attach_remove_links<R, T, Rel, P, C>(router: &mut R)
where
    R: Router,
    T: RemoveLinks<Rel>,
    Rel: ToMany,
    Rel::Resource: RawResource,
    P: Presenter<(), R>,
    C: Receiver<Rel::Resource, R::Request>,
{
    super::attach::<R, T>(router, ResourceRoute {
        method: Method::Remove,
        relation: Some((Rel::to_many(), true))
    }, remove_links::<R, T, Rel, P, C>);
}

fn remove_links<R, T, Rel, P, C>(request: R::Request, link_maker: R::LinkMaker) -> Box<Future<Item = R::Response, Error = ()>>
where
    R: Router,
    T: RemoveLinks<Rel>,
    Rel: ToMany,
    Rel::Resource: RawResource,
    P: Presenter<(), R>,
    C: Receiver<Rel::Resource, R::Request>,
{
    let presenter = P::prepare(None, link_maker);
    let parsed_id = match request.id() {
        Some(id)    => try_status!(id.parse(), presenter),
        None        => try_status!(Err(()), presenter),
    };
    let identifiers = try_status!(C::receive_identifiers(request), presenter);
    let parsed_rel_ids = try_status!(identifiers.into_iter().map(|identifier| identifier.id.parse()).collect::<Result<Vec<_>, _>>(), presenter);
    presenter.try_present(T::remove_links(Entity::Id(parsed_id), &parsed_rel_ids).into_future().map(|_| ()))
}
