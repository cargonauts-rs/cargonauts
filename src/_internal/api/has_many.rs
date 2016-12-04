use api::{Resource, Error, Entity};
use api::raw::{Relationship, Identifier, RelResponse};
use api::rel::{ToMany, HasMany};
use router::{Router, Component, Method, Request};
use presenter::Presenter;
use Future;
use IntoFuture;

pub trait _MaybeAttachHasMany<Rel: ToMany, P, R: Router> {
    fn attach(_: &mut R) { }
}

impl<T: Resource, Rel: ToMany, P, R: Router> _MaybeAttachHasMany<Rel, P, R> for T { }

impl<T, Rel, P, R> _MaybeAttachHasMany<Rel, P, R> for T
where
    T: HasMany<Rel>,
    Rel: ToMany,
    P: Presenter<(), R>,
    R: Router,
{
    fn attach(router: &mut R) {
        _attach_has_many::<R, T, Rel, P>(router);
    }
}

pub fn _attach_has_many<R, T, Rel, P>(router: &mut R)
where
    T: HasMany<Rel>,
    Rel: ToMany,
    P: Presenter<(), R>,
    R: Router,
{
    super::attach::<R, T>(
        router,
        Method::Read,
        Component::Relationship(Rel::to_many()),
        has_many::<R, T, Rel, P>
    );
}

fn has_many<R, T, Rel, P>(request: R::Request, link_maker: R::LinkMaker) -> Box<Future<Item = R::Response, Error = ()>>
where
    R: Router,
    T: HasMany<Rel>,
    Rel: ToMany,
    P: Presenter<(), R>,
{
    let options = request.collection_options();
    let presenter = P::prepare(options.field_set, link_maker);
    let parsed_id = match request.id() {
        Some(id)    => try_status!(id.parse(), presenter),
        None        => try_status!(Err(()), presenter),
    };
    presenter.try_present(T::has_many(Entity::Id(parsed_id)).into_future().map(move |rel| {
        RelResponse {
            resource: T::resource_plural(),
            related: Rel::to_many(),
            id: request.id().unwrap().to_owned(),
            rel: Relationship::Many(rel.into_iter().map(|id| Identifier::new::<Rel::Resource>(&id)).collect()),
            includes: vec![],
        }
    }))
}
