use api::{Resource, Error};
use api::async::raw::RawPostAsync;
use api::raw::RawPost;
use presenter::Presenter;
use receiver::{Receiver, Post};
use router::{Router, ResourceRoute, Method, Request};
use IntoFuture;
use Future;

pub trait _MaybePost<P, C, R: Router>: Resource {
    fn attach(_: &mut R) { }
}

impl<T: Resource, P, C, R: Router> _MaybePost<P, C, R> for T { }

impl<T, P, C, R> _MaybePost<P, C, R> for T
where
    T: RawPost<P::Include>,
    P: Presenter<T, R>,
    C: Receiver<T, R::Request>,
    R: Router,
{
    fn attach(router: &mut R) {
        _attach_post::<R, T, P, C>(router);
    }
}

pub trait _MaybePostAsync<P, C, R: Router>: Resource {
    fn attach(_: &mut R) { }
}


impl<T: Resource, P, C, R: Router> _MaybePostAsync<P, C, R> for T { }

impl<T, P, C, R> _MaybePostAsync<P, C, R> for T
where
    T: RawPostAsync,
    P: Presenter<T::Job, R>,
    C: Receiver<T, R::Request>,
    R: Router,
{
    fn attach(router: &mut R) {
        _attach_post_async::<R, T, P, C>(router);
    }
}

pub fn _attach_post<R, T, P, C>(router: &mut R)
where
    R: Router,
    T: RawPost<P::Include>,
    P: Presenter<T, R>,
    C: Receiver<T, R::Request>,
{
    super::attach::<R, T>(router, ResourceRoute {
        method: Method::Post,
        relation: None,
    }, post::<R, T, P, C>);
}

pub fn _attach_post_async<R, T, P, C>(router: &mut R)
where
    R: Router,
    T: RawPostAsync,
    P: Presenter<T::Job, R>,
    C: Receiver<T, R::Request>,
{
    super::attach::<R, T>(router, ResourceRoute {
        method: Method::Post,
        relation: None,
    }, post_async::<R, T, P, C>);
}

fn post<R, T, P, C>(request: R::Request, link_maker: R::LinkMaker) -> Box<Future<Item = R::Response, Error = ()>>
where
    R: Router,
    T: RawPost<P::Include>,
    P: Presenter<T, R>,
    C: Receiver<T, R::Request>,
{
    let options = request.resource_options();
    let presenter = P::prepare(options.field_set, link_maker);
    let received = try_status!(C::receive_post(request), presenter);
    match received {
        Post::One(data)     => presenter.try_present(T::post(data)),
        Post::Many(data)    => presenter.try_present(T::append(data)),
    }
}

fn post_async<R, T, P, C>(request: R::Request, link_maker: R::LinkMaker) -> Box<Future<Item = R::Response, Error = ()>>
where
    R: Router,
    T: RawPostAsync,
    P: Presenter<T::Job, R>,
    C: Receiver<T, R::Request>,
{
    let presenter = P::prepare(None, link_maker);
    let received = try_status!(C::receive_post(request), presenter);
    match received {
        Post::One(data)     => presenter.try_present(T::post_async(data)),
        Post::Many(_)       => Box::new(Ok(presenter.present_err(Error::BadRequest)).into_future()),
    }
}
