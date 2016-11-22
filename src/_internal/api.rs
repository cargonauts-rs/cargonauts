use std::io::Read;

use api;
use api::async;
use api::raw;
use presenter::Presenter;
use receiver::{Receiver, PatchReceiver};
use router::Router;
use _internal::_Router;

pub trait _MaybeGet<P, R: Router> {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: api::Resource, P, R: Router> _MaybeGet<P, R> for T { }

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

pub trait _MaybeIndex<P, R: Router> {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: api::Resource, P, R: Router> _MaybeIndex<P, R> for T { }

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

pub trait _MaybeDelete<P, R: Router> {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: api::Resource, P, R: Router> _MaybeDelete<P, R> for T { }

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

pub trait _MaybeClear<P, R: Router> {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: api::Resource, P, R: Router> _MaybeClear<P, R> for T { }

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

pub trait _MaybeRemove<P, R: Router> {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: api::Resource, P, R: Router> _MaybeRemove<P, R> for T { }

impl<T, P, R> _MaybeRemove<P, R> for T
where
    T: api::Remove,
    P: Presenter<(), R>,
    R: Router,
{
    fn attach(router: &mut _Router<R>) {
        router.attach_remove::<T, P>();
    }
}

pub trait _MaybePatch<P, C, R: Router> {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: api::Resource, P, C, R: Router> _MaybePatch<P, C, R> for T { }

impl<T, P, C, R> _MaybePatch<P, C, R> for T
where
    T: raw::RawPatch<P::Include>,
    P: Presenter<T, R>,
    C: PatchReceiver<T, Box<Read>, raw::Synchronous>,
    R: Router,
{
    fn attach(router: &mut _Router<R>) {
        router.attach_patch::<T, P, C>();
    }
}

pub trait _MaybePatchAsync<P, C, R: Router> {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: api::Resource, P, C, R: Router> _MaybePatchAsync<P, C, R> for T { }

impl<T, P, C, R> _MaybePatchAsync<P, C, R> for T
where
    T: async::raw::RawPatchAsync,
    P: Presenter<T::Job, R>,
    C: PatchReceiver<T, Box<Read>, async::raw::Asynchronous>,
    R: Router,
{
    fn attach(router: &mut _Router<R>) {
        router.attach_patch_async::<T, P, C>();
    }
}

pub trait _MaybePost<P, C, R: Router> {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: api::Resource, P, C, R: Router> _MaybePost<P, C, R> for T { }

impl<T, P, C, R> _MaybePost<P, C, R> for T
where
    T: raw::RawPost<P::Include>,
    P: Presenter<T, R>,
    C: Receiver<T, Box<Read>>,
    R: Router,
{
    fn attach(router: &mut _Router<R>) {
        router.attach_post::<T, P, C>();
    }
}

pub trait _MaybePostAsync<P, C, R: Router> {
    fn attach(_: &mut _Router<R>) { }
}


impl<T: api::Resource, P, C, R: Router> _MaybePostAsync<P, C, R> for T { }

impl<T, P, C, R> _MaybePostAsync<P, C, R> for T
where
    T: async::raw::RawPostAsync,
    P: Presenter<T::Job, R>,
    C: Receiver<T, Box<Read>>,
    R: Router,
{
    fn attach(router: &mut _Router<R>) {
        router.attach_post_async::<T, P, C>();
    }
}

pub trait _MaybeAppend<P, C, R: Router> {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: api::Resource, P, C, R: Router> _MaybeAppend<P, C, R> for T { }

impl<T, P, C, R> _MaybeAppend<P, C, R> for T
where
    T: raw::RawAppend<P::Include>,
    P: Presenter<T, R>,
    C: Receiver<T, Box<Read>>,
    R: Router,
{
    fn attach(router: &mut _Router<R>) {
        router.attach_append::<T, P, C>();
    }
}

pub trait _MaybeReplace<P, C, R: Router> {
    fn attach(_: &mut _Router<R>) { }
}

impl<T: api::Resource, P, C, R: Router> _MaybeReplace<P, C, R> for T { }

impl<T, P, C, R> _MaybeReplace<P, C, R> for T
where
    T: raw::RawReplace<P::Include>,
    P: Presenter<T, R>,
    C: Receiver<T, Box<Read>>,
    R: Router,
{
    fn attach(router: &mut _Router<R>) {
        router.attach_replace::<T, P, C>();
    }
}
