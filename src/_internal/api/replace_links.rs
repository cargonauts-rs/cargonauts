use api::{Resource, Error, Entity};
use api::rel::ToMany;
use api::rel::ReplaceLinks;
use receiver::Receiver;
use router::{Router, Component, Method, Request};
use presenter::Presenter;
use Future;
use IntoFuture;

pub trait _MaybeAttachReplaceLinks<Rel: ToMany, P, C, R: Router>: Resource {
    fn attach(_: &mut R) { }
}

impl<T: Resource, Rel: ToMany, C, P, R: Router> _MaybeAttachReplaceLinks<Rel, P, C, R> for T { }

impl<T, Rel, C, P, R> _MaybeAttachReplaceLinks<Rel, P, C, R> for T
where
    T: ReplaceLinks<Rel>,
    Rel: ToMany,
    P: Presenter<(), R>,
    C: Receiver<(), R::Request>,
    R: Router,
{
    fn attach(router: &mut R) {
        _attach_replace_links::<R, T, Rel, P, C>(router);

    }
}

pub fn _attach_replace_links<R, T, Rel, P, C>(router: &mut R)
where
    R: Router,
    T: ReplaceLinks<Rel>,
    Rel: ToMany,
    P: Presenter<(), R>,
    C: Receiver<(), R::Request>,
{
    super::attach::<R, T>(
        router,
        Method::Update,
        Component::Relationship(Rel::to_many()),
        replace_links::<R, T, Rel, P, C>
    );
}

fn replace_links<R, T, Rel, P, C>(request: R::Request, link_maker: R::LinkMaker) -> Box<Future<Item = R::Response, Error = ()>>
where
    R: Router,
    T: ReplaceLinks<Rel>,
    Rel: ToMany,
    P: Presenter<(), R>,
    C: Receiver<(), R::Request>,
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
    let identifiers = match C::receive_to_many::<Rel>(request) {
        Ok(rel) => rel,
        Err(_)  => return Box::new(Ok(<P as Presenter<(), R>>::present_err(presenter, Error::Conflict)).into_future()),
    };
    let rel_ids: Vec<_> = {
        let mut ids = vec![];
        for identifier in identifiers {
            match identifier.id.parse() {
                Ok(id)  => ids.push(id),
                Err(_)  => return Box::new(Ok(<P as Presenter<(), R>>::present_err(presenter, Error::Conflict)).into_future()),
            }
        }
        ids
    };
    presenter.try_present(T::replace_links(Entity::Id(id), &rel_ids))
}
