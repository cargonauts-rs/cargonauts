macro_rules! try_status {
    ($x:expr, $p:expr)  => {
        match $x {
            Ok(x)   => x,
            Err(_)  => {
                return Box::new(Ok($p.present_err(Error::Conflict)).into_future()) as Box<Future<Item = R::Response, Error = ()>>
            }
        }
    };
}

mod delete;
mod get;
mod index;
mod patch;
mod post;
mod remove;
mod replace;

pub use self::delete::{_attach_delete, _MaybeDelete};
pub use self::get::{_attach_get, _MaybeGet};
pub use self::index::{_attach_index, _MaybeIndex};
pub use self::post::{_attach_post, _attach_post_async, _MaybePost, _MaybePostAsync};
pub use self::patch::{_attach_patch, _attach_patch_async, _MaybePatch, _MaybePatchAsync};
pub use self::remove::{_attach_remove, _MaybeRemove};
pub use self::replace::{_attach_replace, _MaybeReplace};

mod has_one;
mod has_many;

pub use self::has_one::{_attach_has_one, _MaybeHasOne};
pub use self::has_many::{_attach_has_many, _MaybeHasMany};

mod get_one;
mod index_many;

pub use self::get_one::{_attach_get_one, _MaybeGetOne};
pub use self::index_many::{_attach_index_many, _MaybeIndexMany};

use api::Resource;
use router::{Router, ResourceRoute};
use Future;

fn attach<R, T>(router: &mut R,
                route: ResourceRoute<'static>,
                handler: fn(R::Request, R::LinkMaker) -> Box<Future<Item = R::Response, Error = ()>>)
where R: Router, T: Resource, {
    router.attach_resource(T::resource_plural(), route, handler)
}

use api::raw;
use api::rel::{self, ToOne, ToMany};
use presenter::Presenter;
use receiver::{Receiver, PatchReceiver};
use _internal::_Router;

pub trait _MaybeDeleteOne<Rel: ToOne, P, R: Router>: Resource {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: Resource, Rel: ToOne, P, R: Router> _MaybeDeleteOne<Rel, P, R> for T { }

impl<T, Rel, P, R> _MaybeDeleteOne<Rel, P, R> for T
where
    T: rel::raw::DeleteOne<Rel>,
    Rel: ToOne,
    P: Presenter<(), R>,
    R: Router,
{
    fn attach(router: &mut _Router<R>) {
        router.attach_delete_one::<T, Rel, P>();
    }
}

pub trait _MaybeRemoveMany<Rel: ToMany, P, C, R: Router>: Resource {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: Resource, Rel: ToMany, P, C, R: Router> _MaybeRemoveMany<Rel, P, C, R> for T { }

impl<T, Rel, P, C, R> _MaybeRemoveMany<Rel, P, C, R> for T
where
    T: rel::raw::RemoveMany<Rel>,
    Rel: ToMany,
    Rel::Resource: raw::RawResource,
    P: Presenter<(), R>,
    C: Receiver<Rel::Resource, R::Request>,
    R: Router,
{
    fn attach(router: &mut _Router<R>) {
        router.attach_remove_many::<T, Rel, P, C>();
    }
}

pub trait _MaybePatchOne<Rel: ToOne, P, C, R: Router>: Resource {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: Resource, Rel: ToOne, C, P, R: Router> _MaybePatchOne<Rel, P, C, R> for T { }

impl<T, Rel, C, P, R> _MaybePatchOne<Rel, P, C, R> for T
where
    T: rel::raw::PatchOne<P::Include, Rel>,
    Rel: ToOne,
    Rel::Resource: raw::RawHasPatch<raw::Synchronous>,
    P: Presenter<Rel::Resource, R>,
    C: PatchReceiver<Rel::Resource, R::Request, raw::Synchronous>,
    R: Router,
{
    fn attach(router: &mut _Router<R>) {
        router.attach_patch_one::<T, Rel, P, C>();
    }
}

pub trait _MaybeReplaceOne<Rel: ToOne, P, C, R: Router>: Resource {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: Resource, Rel: ToOne, C, P, R: Router> _MaybeReplaceOne<Rel, P, C, R> for T { }

impl<T, Rel, C, P, R> _MaybeReplaceOne<Rel, P, C, R> for T
where
    T: rel::LinkOne<Rel> + rel::UnlinkOne<Rel>,
    Rel: ToOne,
    Rel::Resource: raw::RawResource,
    P: Presenter<Rel::Resource, R> + Presenter<(), R>,
    C: Receiver<Rel::Resource, R::Request>,
    R: Router,
{
    fn attach(router: &mut _Router<R>) {
        router.attach_replace_one::<T, Rel, P, C>();
    }
}

pub trait _MaybeAppendMany<Rel: ToMany, P, C, R: Router>: Resource {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: Resource, Rel: ToMany, C, P, R: Router> _MaybeAppendMany<Rel, P, C, R> for T { }

impl<T, Rel, C, P, R> _MaybeAppendMany<Rel, P, C, R> for T
where
    T: rel::raw::AppendMany<<P as Presenter<Rel::Resource, R>>::Include, Rel>,
    Rel: ToMany,
    Rel::Resource: raw::RawResource,
    P: Presenter<Rel::Resource, R> + Presenter<(), R>,
    C: Receiver<Rel::Resource, R::Request>,
    R: Router,
{
    fn attach(router: &mut _Router<R>) {
        router.attach_append_many::<T, Rel, P, C>();
    }
}

pub trait _MaybeReplaceMany<Rel: ToMany, P, C, R: Router>: Resource {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: Resource, Rel: ToMany, C, P, R: Router> _MaybeReplaceMany<Rel, P, C, R> for T { }

impl<T, Rel, C, P, R> _MaybeReplaceMany<Rel, P, C, R> for T
where
    T: rel::raw::ReplaceMany<<P as Presenter<Rel::Resource, R>>::Include, Rel>,
    Rel: ToMany,
    Rel::Resource: raw::RawResource,
    P: Presenter<Rel::Resource, R> + Presenter<(), R>,
    C: Receiver<Rel::Resource, R::Request>,
    R: Router,
{
    fn attach(router: &mut _Router<R>) {
        router.attach_replace_many::<T, Rel, P, C>();
    }
}
