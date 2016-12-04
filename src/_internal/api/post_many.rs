use api::{Resource, Error, Entity};
use api::raw::RawResource;
use api::rel::ToMany;
use api::rel::raw::PostMany;
use receiver::{Receiver, Post};
use router::{Router, Component, Method, Request};
use presenter::Presenter;
use Future;
use IntoFuture;

pub trait _MaybePostMany<Rel: ToMany, P, C, R: Router>: Resource {
    fn attach(_: &mut R) { }
}

impl<T: Resource, Rel: ToMany, C, P, R: Router> _MaybePostMany<Rel, P, C, R> for T { }

impl<T, Rel, C, P, R> _MaybePostMany<Rel, P, C, R> for T
where
    T: PostMany<P::Include, Rel>,
    Rel: ToMany,
    Rel::Resource: RawResource,
    P: Presenter<Rel::Resource, R>,
    C: Receiver<Rel::Resource, R::Request>,
    R: Router,
{
    fn attach(router: &mut R) {
        _attach_post_many::<R, T, Rel, P, C>(router);
    }
}

pub fn _attach_post_many<R, T, Rel, P, C>(router: &mut R)
where
    R: Router,
    T: PostMany<P::Include, Rel>,
    Rel: ToMany,
    Rel::Resource: RawResource,
    P: Presenter<Rel::Resource, R>,
    C: Receiver<Rel::Resource, R::Request>,
{
    super::attach::<R, T>(
        router,
        Method::Create,
        Component::Related(Rel::to_many()),
        post_many::<R, T, Rel, P, C>
    );
}

fn post_many<R, T, Rel, P, C>(request: R::Request, link_maker: R::LinkMaker) -> Box<Future<Item = R::Response, Error = ()>>
where
    R: Router,
    T: PostMany<P::Include, Rel>,
    Rel: ToMany,
    Rel::Resource: RawResource,
    P: Presenter<Rel::Resource, R>,
    C: Receiver<Rel::Resource, R::Request>,
{
    let options = request.collection_options();
    let presenter = P::prepare(options.field_set, link_maker);
    let id = match request.id() {
        Some(id)    => try_status!(id.parse(), presenter),
        None        => try_status!(Err(()), presenter),
    };
    let received = try_status!(C::receive_post(request), presenter);
    match received {
        Post::One(data)     => presenter.try_present(T::post_one(Entity::Id(id), data)),
        Post::Many(data)    => presenter.try_present(T::append_many(Entity::Id(id), data))
    }
}
