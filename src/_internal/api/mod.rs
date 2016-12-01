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

pub use self::delete::{_attach_delete, _MaybeDelete as _MaybeAttachDelete};
pub use self::get::{_attach_get, _MaybeGet as _MaybeAttachGet};
pub use self::index::{_attach_index, _MaybeIndex as _MaybeAttachIndex};
pub use self::post::{_attach_post, _attach_post_async, _MaybePost as _MaybeAttachPost, _MaybePostAsync as _MaybeAttachPostAsync};
pub use self::patch::{_attach_patch, _attach_patch_async, _MaybePatch as _MaybeAttachPatch, _MaybePatchAsync as _MaybeAttachPatchAsync};
pub use self::remove::{_attach_remove, _MaybeRemove as _MaybeAttachRemove};
pub use self::replace::{_attach_replace, _MaybeReplace as _MaybeAttachReplace};

mod has_one;
mod has_many;
mod remove_links;
mod update_link;

pub use self::has_one::{_attach_has_one, _MaybeHasOne as _MaybeAttachHasOne};
pub use self::has_many::{_attach_has_many, _MaybeHasMany as _MaybeAttachHasMany};
pub use self::remove_links::{_attach_remove_links, _MaybeRemoveLinks as _MaybeAttachRemoveLinks};
pub use self::update_link::{_attach_update_link, _MaybeUpdateLink as _MaybeAttachUpdateLink};

mod delete_one;
mod get_one;
mod index_many;
mod remove_many;

pub use self::delete_one::{_attach_delete_one, _MaybeDeleteOne as _MaybeAttachDeleteOne};
pub use self::get_one::{_attach_get_one, _MaybeGetOne as _MaybeAttachGetOne};
pub use self::index_many::{_attach_index_many, _MaybeIndexMany as _MaybeAttachIndexMany};
pub use self::remove_many::{_attach_remove_many, _MaybeRemoveMany as _MaybeAttachRemoveMany};

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
