use api::{self, Resource};
use api::async;
use api::raw;
use api::rel::{self, ToOne, ToMany};
use presenter::Presenter;
use receiver::{Receiver, PatchReceiver, IdReceiver};
use router::Router;
use _internal::_Router;

pub trait _MaybeGet<P, R: Router>: Resource {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: Resource, P, R: Router> _MaybeGet<P, R> for T { }

impl<T, P, R> _MaybeGet<P, R> for T
where
    T: raw::RawGet<P::Include>,
    P: Presenter<T, R>,
    R: Router,
{
    fn attach(router: &mut _Router<R>) {
        router.attach_get::<T, P>();
    }
}

pub trait _MaybeIndex<P, R: Router>: Resource {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: Resource, P, R: Router> _MaybeIndex<P, R> for T { }

impl<T, P, R> _MaybeIndex<P, R> for T
where
    T: raw::RawIndex<P::Include>,
    P: Presenter<T, R>,
    R: Router,
{
    fn attach(router: &mut _Router<R>) {
        router.attach_index::<T, P>();
    }
}

pub trait _MaybeDelete<P, R: Router>: Resource {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: Resource, P, R: Router> _MaybeDelete<P, R> for T { }

impl<T, P, R> _MaybeDelete<P, R> for T
where
    T: api::Delete,
    P: Presenter<(), R>,
    R: Router,
{
    fn attach(router: &mut _Router<R>) {
        router.attach_delete::<T, P>();
    }
}

pub trait _MaybeClear<P, R: Router>: Resource {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: Resource, P, R: Router> _MaybeClear<P, R> for T { }

impl<T, P, R> _MaybeClear<P, R> for T
where
    T: api::Clear,
    P: Presenter<(), R>,
    R: Router,
{
    fn attach(router: &mut _Router<R>) {
        router.attach_clear::<T, P>();
    }
}

pub trait _MaybeRemove<P, C, R: Router>: Resource {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: Resource, P, C, R: Router> _MaybeRemove<P, C, R> for T { }

impl<T, P, C, R> _MaybeRemove<P, C, R> for T
where
    T: api::Remove,
    P: Presenter<(), R>,
    C: IdReceiver<T, R::Request>,
    R: Router,
{
    fn attach(router: &mut _Router<R>) {
        router.attach_remove::<T, P, C>();
    }
}

pub trait _MaybePatch<P, C, R: Router>: Resource {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: Resource, P, C, R: Router> _MaybePatch<P, C, R> for T { }

impl<T, P, C, R> _MaybePatch<P, C, R> for T
where
    T: raw::RawPatch<P::Include>,
    P: Presenter<T, R>,
    C: PatchReceiver<T, R::Request, raw::Synchronous>,
    R: Router,
{
    fn attach(router: &mut _Router<R>) {
        router.attach_patch::<T, P, C>();
    }
}

pub trait _MaybePatchAsync<P, C, R: Router>: Resource {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: Resource, P, C, R: Router> _MaybePatchAsync<P, C, R> for T { }

impl<T, P, C, R> _MaybePatchAsync<P, C, R> for T
where
    T: async::raw::RawPatchAsync,
    P: Presenter<T::Job, R>,
    C: PatchReceiver<T, R::Request, async::raw::Asynchronous>,
    R: Router,
{
    fn attach(router: &mut _Router<R>) {
        router.attach_patch_async::<T, P, C>();
    }
}

pub trait _MaybePost<P, C, R: Router>: Resource {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: Resource, P, C, R: Router> _MaybePost<P, C, R> for T { }

impl<T, P, C, R> _MaybePost<P, C, R> for T
where
    T: raw::RawPost<P::Include>,
    P: Presenter<T, R>,
    C: Receiver<T, R::Request>,
    R: Router,
{
    fn attach(router: &mut _Router<R>) {
        router.attach_post::<T, P, C>();
    }
}

pub trait _MaybePostAsync<P, C, R: Router>: Resource {
    fn attach(_: &mut _Router<R>) { }
}


impl<T: Resource, P, C, R: Router> _MaybePostAsync<P, C, R> for T { }

impl<T, P, C, R> _MaybePostAsync<P, C, R> for T
where
    T: async::raw::RawPostAsync,
    P: Presenter<T::Job, R>,
    C: Receiver<T, R::Request>,
    R: Router,
{
    fn attach(router: &mut _Router<R>) {
        router.attach_post_async::<T, P, C>();
    }
}

pub trait _MaybeAppend<P, C, R: Router>: Resource {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: Resource, P, C, R: Router> _MaybeAppend<P, C, R> for T { }

impl<T, P, C, R> _MaybeAppend<P, C, R> for T
where
    T: raw::RawAppend<P::Include>,
    P: Presenter<T, R>,
    C: Receiver<T, R::Request>,
    R: Router,
{
    fn attach(router: &mut _Router<R>) {
        router.attach_append::<T, P, C>();
    }
}

pub trait _MaybeReplace<P, C, R: Router>: Resource {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: Resource, P, C, R: Router> _MaybeReplace<P, C, R> for T { }

impl<T, P, C, R> _MaybeReplace<P, C, R> for T
where
    T: raw::RawReplace<P::Include>,
    P: Presenter<T, R>,
    C: Receiver<T, R::Request>,
    R: Router,
{
    fn attach(router: &mut _Router<R>) {
        router.attach_replace::<T, P, C>();
    }
}

pub trait _MaybeGetOne<Rel: ToOne, P, R: Router>: Resource {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: Resource, Rel: ToOne, P, R: Router> _MaybeGetOne<Rel, P, R> for T { }

impl<T, Rel, P, R> _MaybeGetOne<Rel, P, R> for T
where
    T: rel::raw::GetOne<P::Include, Rel>,
    Rel: ToOne,
    Rel::Resource: raw::RawFetch,
    P: Presenter<Rel::Resource, R>,
    R: Router,
{
    fn attach(router: &mut _Router<R>) {
        router.attach_fetch_one::<T, Rel, P>();
    }
}

pub trait _MaybeIndexMany<Rel: ToMany, P, R: Router>: Resource {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: Resource, Rel: ToMany, P, R: Router> _MaybeIndexMany<Rel, P, R> for T { }

impl<T, Rel, P, R> _MaybeIndexMany<Rel, P, R> for T
where
    T: rel::raw::IndexMany<P::Include, Rel>,
    Rel: ToMany,
    Rel::Resource: raw::RawFetch,
    P: Presenter<Rel::Resource, R>,
    R: Router,
{
    fn attach(router: &mut _Router<R>) {
        router.attach_fetch_many::<T, Rel, P>();
    }
}

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

pub trait _MaybeClearMany<Rel: ToMany, P, R: Router>: Resource {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: Resource, Rel: ToMany, P, R: Router> _MaybeClearMany<Rel, P, R> for T { }

impl<T, Rel, P, R> _MaybeClearMany<Rel, P, R> for T
where
    T: rel::raw::ClearMany<Rel>,
    Rel: ToMany,
    P: Presenter<(), R>,
    R: Router,
{
    fn attach(router: &mut _Router<R>) {
        router.attach_clear_many::<T, Rel, P>();
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
    P: Presenter<(), R>,
    C: IdReceiver<Rel::Resource, R::Request>,
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

pub trait _MaybePostOne<Rel: ToOne, P, C, R: Router>: Resource {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: Resource, Rel: ToOne, C, P, R: Router> _MaybePostOne<Rel, P, C, R> for T { }

impl<T, Rel, C, P, R> _MaybePostOne<Rel, P, C, R> for T
where
    T: rel::raw::PostOne<<P as Presenter<Rel::Resource, R>>::Include, Rel>,
    Rel: ToOne,
    Rel::Resource: raw::RawUpdate,
    P: Presenter<Rel::Resource, R> + Presenter<(), R>,
    C: Receiver<Rel::Resource, R::Request>,
    R: Router,
{
    fn attach(router: &mut _Router<R>) {
        router.attach_post_one::<T, Rel, P, C>();
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
    Rel::Resource: raw::RawUpdate,
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
    Rel::Resource: raw::RawUpdate,
    P: Presenter<Rel::Resource, R> + Presenter<(), R>,
    C: Receiver<Rel::Resource, R::Request>,
    R: Router,
{
    fn attach(router: &mut _Router<R>) {
        router.attach_replace_many::<T, Rel, P, C>();
    }
}
