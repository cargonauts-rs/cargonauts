#![allow(unused_parens)]
use std::io::Read;

use api::{self, Error};
use api::async;
use api::rel;
use api::raw;
use router::{self as r, Router};
use Deserialize;
use futures::{IntoFuture, Future};
use receiver::{Receiver, PatchReceiver};
use presenter::Presenter;

macro_rules! try_status {
    ($x:expr, $p:expr)  => {
        match $x {
            Ok(x)   => x,
            Err(_)  => {
                return $p.present_err(Error::Conflict)
            }
        }
    };
}

pub struct _Router<'a, R: Router + 'a> {
    router: &'a mut R,
}

impl<'a, R: Router> _Router<'a, R> {
    pub fn new(router: &'a mut R) -> _Router<'a, R> {
        _Router {
            router: router,
        }
    }

    pub fn attach_get<T, P>(&mut self)
    where
        T: raw::RawGet<P::Include>,
        P: Presenter<T, R>,
    {
        fn get<R, T, P>(request: r::GetRequest, link_maker: R::LinkMaker) -> R::Response
        where
            T: raw::RawGet<P::Include>,
            P: Presenter<T, R>,
            R: Router,
        {
            let presenter = P::prepare(request.field_set, link_maker);
            let id = try_status!(request.id.parse(), presenter);
            presenter.try_present(T::get_id(id, &request.includes).into_future().wait())
        }
        self.router.attach_get(T::resource_plural(), get::<R, T, P>);
    }

    pub fn attach_index<T, P>(&mut self)
    where
        T: raw::RawIndex<P::Include>,
        P: Presenter<T, R>,
    {
        fn index<R, T, P>(request: r::IndexRequest, link_maker: R::LinkMaker) -> R::Response
        where 
            R: Router,
            T: raw::RawIndex<P::Include>,
            P: Presenter<T, R>,
        {
            let presenter = P::prepare(request.field_set, link_maker);
            presenter.try_present(T::index(&request.includes, &request.sort, &request.page).into_future().wait())
        }
        self.router.attach_index(T::resource_plural(), index::<R, T, P>);
    }

    pub fn attach_delete<T, P>(&mut self)
    where
        T: api::Delete,
        P: Presenter<(), R>,
    {
        fn delete<R, T, P>(request: r::DeleteRequest, link_maker: R::LinkMaker) -> R::Response
        where
            T: api::Delete,
            P: Presenter<(), R>,
            R: Router,
        {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(request.id.parse(), presenter);
            presenter.try_present(T::delete(&id).into_future().wait())
        }
        self.router.attach_delete(T::resource_plural(), delete::<R, T, P>);
    }

    pub fn attach_clear<T, P>(&mut self)
    where
        T: api::Clear,
        P: Presenter<(), R>,
    {
        fn clear<R, T, P>(link_maker: R::LinkMaker) -> R::Response
        where
            T: api::Clear,
            P: Presenter<(), R>,
            R: Router,
        {
            let presenter = P::prepare(None, link_maker);
            presenter.try_present(T::clear().into_future().wait())
        }
        self.router.attach_clear(T::resource_plural(), clear::<R, T, P>);
    }

    pub fn attach_remove<T, P>(&mut self)
    where
        T: api::Remove,
        P: Presenter<(), R>,
    {
        fn remove<R, T, P>(request: r::RemoveRequest, link_maker: R::LinkMaker) -> R::Response
        where
            T: api::Remove,
            P: Presenter<(), R>,
            R: Router,
        {
            let presenter = P::prepare(None, link_maker);
            let ids: Vec<_> = try_status!(request.ids.iter().map(|id| id.parse()).collect::<Result<Vec<_>, _>>(), presenter);
            presenter.try_present(T::remove(&ids).into_future().wait())
        }
        self.router.attach_remove(T::resource_plural(), remove::<R, T, P>);
    }

    pub fn attach_patch<T, P, C>(&mut self)
    where
        T: raw::RawPatch<P::Include>,
        P: Presenter<T, R>,
        C: PatchReceiver<T, Box<Read>, raw::Synchronous>,
    {
        fn patch<R, T, P, C>(request: r::PatchRequest, link_maker: R::LinkMaker) -> R::Response
        where
            R: Router,
            T: raw::RawPatch<P::Include>,
            P: Presenter<T, R>,
            C: PatchReceiver<T, Box<Read>, raw::Synchronous>,
        {
            let presenter = P::prepare(request.field_set, link_maker);
            let id = try_status!(request.id.parse(), presenter);
            let received = try_status!(C::wrap(request.body).receive_patch(), presenter);
            presenter.try_present(T::patch(id, received).into_future().wait())
        }
        self.router.attach_patch(T::resource_plural(), patch::<R, T, P, C>);
    }

    pub fn attach_patch_async<T, P, C>(&mut self)
    where
        T: async::raw::RawPatchAsync,
        P: Presenter<T::Job, R>,
        C: PatchReceiver<T, Box<Read>, async::raw::Asynchronous>,
    {
        fn patch_async<R, T, P, C>(request: r::PatchRequest, link_maker: R::LinkMaker) -> R::Response
        where
            T: async::raw::RawPatchAsync,
            P: Presenter<T::Job, R>,
            R: Router,
            C: PatchReceiver<T, Box<Read>, async::raw::Asynchronous>,
        {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(request.id.parse(), presenter);
            let received = try_status!(C::wrap(request.body).receive_patch(), presenter);
            presenter.try_present(T::patch_async(id, received).into_future().wait())
        }
        self.router.attach_patch(T::resource_plural(), patch_async::<R, T, P, C>);
    }

    pub fn attach_post<T, P, C>(&mut self)
    where
        T: raw::RawPost<P::Include>,
        P: Presenter<T, R>,
        C: Receiver<T, Box<Read>>,
    {
        fn post<R, T, P, C>(request: r::PostRequest, link_maker: R::LinkMaker) -> R::Response
        where
            R: Router,
            T: raw::RawPost<P::Include>,
            P: Presenter<T, R>,
            C: Receiver<T, Box<Read>>,
        {
            let presenter = P::prepare(request.field_set, link_maker);
            let received = try_status!(C::wrap(request.body).receive_resource(), presenter);
            presenter.try_present(T::post(received).into_future().wait())
        }
        self.router.attach_post(T::resource_plural(), post::<R, T, P, C>);
    }

    pub fn attach_post_async<T, P, C>(&mut self)
    where
        T: async::raw::RawPostAsync,
        P: Presenter<T::Job, R>,
        C: Receiver<T, Box<Read>>,
    {
        fn post_async<R, T, P, C>(request: r::PostRequest, link_maker: R::LinkMaker) -> R::Response
        where
            R: Router,
            T: async::raw::RawPostAsync,
            P: Presenter<T::Job, R>,
            C: Receiver<T, Box<Read>>,
        {
            let presenter = P::prepare(None, link_maker);
            let received = try_status!(C::wrap(request.body).receive_resource(), presenter);
            presenter.try_present(T::post_async(received).into_future().wait())
        }
        self.router.attach_post(T::resource_plural(), post_async::<R, T, P, C>);
    }

    pub fn attach_append<T, P, C>(&mut self)
    where
        T: raw::RawAppend<P::Include>,
        P: Presenter<T, R>,
        C: Receiver<T, Box<Read>>,
    {
        fn append<R, T, P, C>(request: r::MultiPostRequest, link_maker: R::LinkMaker) -> R::Response
        where
            R: Router,
            T: raw::RawAppend<P::Include>,
            P: Presenter<T, R>,
            C: Receiver<T, Box<Read>>,
        {
            let presenter = P::prepare(None, link_maker);
            let received = try_status!(C::wrap(request.body).receive_collection(), presenter);
            presenter.try_present(T::append(received).into_future().wait())
        }
        self.router.attach_append(T::resource_plural(), append::<R, T, P, C>);
    }

    pub fn attach_replace<T, P, C>(&mut self)
    where
        T: raw::RawReplace<P::Include>,
        P: Presenter<T, R>,
        C: Receiver<T, Box<Read>>,
    {
        fn replace<R, T, P, C>(request: r::MultiPostRequest, link_maker: R::LinkMaker) -> R::Response
        where
            T: raw::RawReplace<P::Include>,
            P: Presenter<T, R>,
            R: Router,
            C: Receiver<T, Box<Read>>,
        {
            let presenter = P::prepare(None, link_maker);
            let received = try_status!(C::wrap(request.body).receive_collection(), presenter);
            presenter.try_present(T::replace(received).into_future().wait())
        }
        self.router.attach_replace(T::resource_plural(), replace::<R, T, P, C>);
    }

    pub fn attach_fetch_one<T, Rel, P>(&mut self)
    where
        T: rel::raw::GetOne<P::Include, Rel>,
        Rel: rel::ToOne,
        Rel::Resource: raw::RawFetch,
        P: Presenter<Rel::Resource, R>,
    {
        fn fetch_one<R, T, Rel, P>(request: r::GetRequest, link_maker: R::LinkMaker) -> R::Response
        where
            T: rel::raw::GetOne<P::Include, Rel>,
            Rel: rel::ToOne,
            Rel::Resource: raw::RawFetch,
            P: Presenter<Rel::Resource, R>,
            R: Router,
        {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(request.id.parse(), presenter);
            presenter.try_present(T::get_one(&api::Entity::Id(id), &request.includes).into_future().wait())
        }
        fn fetch_one_rel<R, T, Rel, P>(request: r::FetchRelRequest, link_maker: R::LinkMaker) -> R::Response
        where
            T: rel::raw::GetOne<P::Include, Rel>,
            Rel: rel::ToOne,
            Rel::Resource: raw::RawFetch,
            P: Presenter<Rel::Resource, R>,
            R: Router,
        {
            let presenter = P::prepare(None, link_maker);
            let parsed_id = try_status!(request.id.parse(), presenter);
            presenter.try_present(T::has_one(&api::Entity::Id(parsed_id)).into_future().wait().map(move |rel| {
                raw::RelResponse {
                    resource: T::resource_plural(),
                    related: Rel::to_one(),
                    id: request.id,
                    rel: raw::Relationship::One(rel.map(|id| raw::Identifier::new::<Rel::Resource>(&id))),
                    includes: vec![],
                }
            }))
        }
        self.router.attach_fetch_one(T::resource_plural(), Rel::to_one(), fetch_one::<R, T, Rel, P>);
        self.router.attach_fetch_rel(T::resource_plural(), Rel::to_one(), fetch_one_rel::<R, T, Rel, P>);
    }

    pub fn attach_fetch_many<T, Rel, P>(&mut self)
    where
        T: rel::raw::IndexMany<P::Include, Rel>,
        Rel: rel::ToMany,
        Rel::Resource: raw::RawFetch,
        P: Presenter<Rel::Resource, R>,
    {
        fn fetch_many<R, T, Rel, P>(request: r::GetRequest, link_maker: R::LinkMaker) -> R::Response
        where
            T: rel::raw::IndexMany<P::Include, Rel>,
            Rel: rel::ToMany,
            Rel::Resource: raw::RawFetch,
            P: Presenter<Rel::Resource, R>,
            R: Router,
        {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(request.id.parse(), presenter);
            presenter.try_present(T::index_many(&api::Entity::Id(id), &request.includes).into_future().wait())
        }
        fn fetch_many_rel<R, T, Rel, P>(request: r::FetchRelRequest, link_maker: R::LinkMaker) -> R::Response
        where 
            T: rel::raw::IndexMany<P::Include, Rel>,
            Rel: rel::ToMany,
            Rel::Resource: raw::RawFetch,
            P: Presenter<Rel::Resource, R>,
            R: Router,
        {
            let presenter = P::prepare(None, link_maker);
            let parsed_id = try_status!(request.id.parse(), presenter);
            presenter.try_present(T::has_many(&api::Entity::Id(parsed_id)).into_future().wait().map(move |rel| {
                raw::RelResponse {
                    resource: T::resource_plural(),
                    related: Rel::to_many(),
                    id: request.id,
                    rel: raw::Relationship::Many(rel.into_iter().map(|id| raw::Identifier::new::<Rel::Resource>(&id)).collect()),
                    includes: vec![],
                }
            }))
        }
        self.router.attach_fetch_many(T::resource_plural(), Rel::to_many(), fetch_many::<R, T, Rel, P>);
        self.router.attach_fetch_rel(T::resource_plural(), Rel::to_many(), fetch_many_rel::<R, T, Rel, P>);
    }

    pub fn attach_delete_one<T, Rel, P>(&mut self)
    where
        T: rel::raw::DeleteOne<Rel>,
        Rel: rel::ToOne,
        P: Presenter<(), R>,
    {
        fn delete_one<R, T, Rel, P>(request: r::DeleteRequest, link_maker: R::LinkMaker) -> R::Response
        where
            T: rel::raw::DeleteOne<Rel>,
            Rel: rel::ToOne,
            P: Presenter<(), R>,
            R: Router,
        {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(request.id.parse(), presenter);
            presenter.try_present(T::delete_one(&api::Entity::Id(id)).into_future().wait())
        }
        fn delete_one_rel<R, T, Rel, P>(request: r::DeleteRequest, link_maker: R::LinkMaker) -> R::Response
        where
            T: rel::raw::DeleteOne<Rel>,
            Rel: rel::ToOne,
            P: Presenter<(), R>,
            R: Router,
        {
            let presenter = P::prepare(None, link_maker);
            let parsed_id = try_status!(request.id.parse(), presenter);
            presenter.try_present(T::unlink_one(&api::Entity::Id(parsed_id)).into_future().wait())
        }
        self.router.attach_delete_one(T::resource_plural(), Rel::to_one(), delete_one::<R, T, Rel, P>);
        self.router.attach_delete_one_rel(T::resource_plural(), Rel::to_one(), delete_one_rel::<R, T, Rel, P>)
    }

    pub fn attach_clear_many<T, Rel, P>(&mut self)
    where
        T: rel::raw::ClearMany<Rel>,
        Rel: rel::ToMany,
        P: Presenter<(), R>,
    {
        fn clear_many<R, T, Rel, P>(request: r::DeleteRequest, link_maker: R::LinkMaker) -> R::Response
        where
            T: rel::raw::ClearMany<Rel>,
            Rel: rel::ToMany,
            P: Presenter<(), R>,
            R: Router,
        {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(request.id.parse(), presenter);
            presenter.try_present(T::clear_many(&api::Entity::Id(id)).into_future().wait())
        }
        fn clear_many_rel<R, T, Rel, P>(request: r::DeleteRequest, link_maker: R::LinkMaker) -> R::Response
        where
            T: rel::raw::ClearMany<Rel>,
            Rel: rel::ToMany,
            P: Presenter<(), R>,
            R: Router,
        {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(request.id.parse(), presenter);
            presenter.try_present(T::clear_links(&api::Entity::Id(id)).into_future().wait())
        }
        self.router.attach_clear_many(T::resource_plural(), Rel::to_many(), clear_many::<R, T, Rel, P>);
        self.router.attach_clear_many_rel(T::resource_plural(), Rel::to_many(), clear_many_rel::<R, T, Rel, P>);
    }

    pub fn attach_remove_many<T, Rel, P>(&mut self)
    where
        T: rel::raw::RemoveMany<Rel>,
        Rel: rel::ToMany,
        P: Presenter<(), R>,
    {
        fn remove_many<R, T, Rel, P>(request: r::RemoveManyRequest, link_maker: R::LinkMaker) -> R::Response
        where
            T: rel::raw::RemoveMany<Rel>,
            Rel: rel::ToMany,
            P: Presenter<(), R>,
            R: Router,
        {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(request.id.parse(), presenter);
            let parsed_rel_ids = try_status!(request.rel_ids.iter().map(|id| id.parse()).collect::<Result<Vec<_>, _>>(), presenter);
            presenter.try_present(T::remove_many(&api::Entity::Id(id), &parsed_rel_ids).into_future().wait())
        }
        fn remove_many_rel<R, T, Rel, P>(request: r::RemoveManyRequest, link_maker: R::LinkMaker) -> R::Response
        where
            T: rel::raw::RemoveMany<Rel>,
            Rel: rel::ToMany,
            P: Presenter<(), R>,
            R: Router,
        {
            let presenter = P::prepare(None, link_maker);
            let parsed_id = try_status!(request.id.parse(), presenter);
            let parsed_rel_ids = try_status!(request.rel_ids.iter().map(|id| id.parse()).collect::<Result<Vec<_>, _>>(), presenter);
            presenter.try_present(T::remove_links(&api::Entity::Id(parsed_id), &parsed_rel_ids).into_future().wait())
        }
        self.router.attach_remove_many(T::resource_plural(), Rel::to_many(), remove_many::<R, T, Rel, P>);
        self.router.attach_remove_many_rel(T::resource_plural(), Rel::to_many(), remove_many_rel::<R, T, Rel, P>);
    }

    pub fn attach_patch_one<T, Rel, P, C>(&mut self)
    where
        T: rel::raw::PatchOne<P::Include, Rel>,
        Rel: rel::ToOne,
        Rel::Resource: raw::RawHasPatch<raw::Synchronous>,
        P: Presenter<Rel::Resource, R>,
        C: PatchReceiver<Rel::Resource, Box<Read>, raw::Synchronous>,
    {
        fn patch_one<R, T, Rel, P, C>(request: r::PatchRequest, link_maker: R::LinkMaker) -> R::Response
        where
            R: Router,
            T: rel::raw::PatchOne<P::Include, Rel>,
            Rel: rel::ToOne,
            Rel::Resource: raw::RawHasPatch<raw::Synchronous>,
            P: Presenter<Rel::Resource, R>,
            C: PatchReceiver<Rel::Resource, Box<Read>, raw::Synchronous>,
        {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(request.id.parse(), presenter);
            let received = try_status!(C::wrap(request.body).receive_patch(), presenter);
            presenter.try_present(T::patch_one(&api::Entity::Id(id), received).into_future().wait())
        }
        self.router.attach_patch_one(T::resource_plural(), Rel::to_one(), patch_one::<R, T, Rel, P, C>);
    }

    pub fn attach_post_one<T, Rel, P, C>(&mut self)
    where
        T: rel::raw::PostOne<<P as Presenter<Rel::Resource, R>>::Include, Rel>,
        Rel: rel::ToOne,
        Rel::Resource: raw::RawUpdate + Deserialize,
        P: Presenter<Rel::Resource, R> + Presenter<(), R>,
        C: Receiver<Rel::Resource, Box<Read>>,
    {
        fn post_one<R, T, Rel, P, C>(request: r::PostOneRequest, link_maker: R::LinkMaker) -> R::Response
        where
            R: Router,
            T: rel::raw::PostOne<P::Include, Rel>,
            Rel: rel::ToOne,
            Rel::Resource: raw::RawUpdate + Deserialize,
            P: Presenter<Rel::Resource, R>,
            C: Receiver<Rel::Resource, Box<Read>>,
        {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(request.id.parse(), presenter);
            let received = try_status!(C::wrap(request.body).receive_resource(), presenter);
            presenter.try_present(T::post_one(&api::Entity::Id(id), received).into_future().wait())
        }
        fn post_one_rel<R, T, Rel, P, C>(request: r::UpdateRelRequest, link_maker: R::LinkMaker) -> R::Response
        where
            R: Router,
            T: rel::raw::PostOne<<P as Presenter<Rel::Resource, R>>::Include, Rel>,
            Rel: rel::ToOne,
            Rel::Resource: raw::RawUpdate + Deserialize,
            P: Presenter<Rel::Resource, R> + Presenter<(), R>,
            C: Receiver<Rel::Resource, Box<Read>>,
        {
            let presenter = <P as Presenter<(), R>>::prepare(None, link_maker);
            let id = match request.id.parse() {
                Ok(id)  => id,
                Err(_)  => return (<P as Presenter<(), R>>::present_err(presenter, Error::Conflict)),
            };
            let rel = match C::wrap(request.body).receive_rel::<Rel>() {
                Ok(rel) => rel,
                Err(_)  => return (<P as Presenter<(), R>>::present_err(presenter, Error::Conflict)),
            };
            let rel_id = match rel {
                raw::Relationship::One(Some(identifier))  => {
                    match identifier.id.parse() {
                        Ok(id)  => id,
                        Err(_)  => return (<P as Presenter<(), R>>::present_err(presenter, Error::Conflict)),
                    }
                },
                _                                   => {
                    return (<P as Presenter<(), R>>::present_err(presenter, Error::BadRequest));
                }
            };
            presenter.try_present(T::link_one(&api::Entity::Id(id), &rel_id).into_future().wait())
        }
        self.router.attach_post_one(T::resource_plural(), Rel::to_one(), post_one::<R, T, Rel, P, C>);
        self.router.attach_update_one_rel(T::resource_plural(), Rel::to_one(), post_one_rel::<R, T, Rel, P, C>);
    }

    pub fn attach_append_many<T, Rel, P, C>(&mut self)
    where
        T: rel::raw::AppendMany<<P as Presenter<Rel::Resource, R>>::Include, Rel>,
        Rel: rel::ToMany,
        Rel::Resource: raw::RawUpdate + Deserialize,
        P: Presenter<Rel::Resource, R> + Presenter<(), R>,
        C: Receiver<Rel::Resource, Box<Read>>,
    {
        fn append_many<R, T, Rel, P, C>(request: r::PostManyRequest, link_maker: R::LinkMaker) -> R::Response
        where
            R: Router,
            T: rel::raw::AppendMany<P::Include, Rel>,
            Rel: rel::ToMany,
            Rel::Resource: raw::RawUpdate + Deserialize,
            P: Presenter<Rel::Resource, R>,
            C: Receiver<Rel::Resource, Box<Read>>,
        {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(request.id.parse(), presenter);
            let received = try_status!(C::wrap(request.body).receive_collection(), presenter);
            presenter.try_present(T::append_many(&api::Entity::Id(id), received).into_future().wait())
        }
        fn append_many_rel<R, T, Rel, P, C>(request: r::UpdateRelRequest, link_maker: R::LinkMaker) -> R::Response
        where
            R: Router,
            T: rel::raw::AppendMany<<P as Presenter<Rel::Resource, R>>::Include, Rel>,
            Rel: rel::ToMany,
            Rel::Resource: raw::RawUpdate + Deserialize,
            P: Presenter<Rel::Resource, R> + Presenter<(), R>,
            C: Receiver<Rel::Resource, Box<Read>>,
        {
            let presenter = <P as Presenter<(), R>>::prepare(None, link_maker);
            let id = match request.id.parse() {
                Ok(id)  => id,
                Err(_)  => return (<P as Presenter<(), R>>::present_err(presenter, Error::Conflict)),
            };
            let rel = match C::wrap(request.body).receive_rel::<Rel>() {
                Ok(rel) => rel,
                Err(_)  => return (<P as Presenter<(), R>>::present_err(presenter, Error::Conflict)),
            };
            let rel_ids: Vec<_> = match rel {
                raw::Relationship::Many(identifiers)   => {
                    let mut ids = vec![];
                    for identifier in identifiers {
                        match identifier.id.parse() {
                            Ok(id)  => ids.push(id),
                            Err(_)  => return (<P as Presenter<(), R>>::present_err(presenter, Error::Conflict)),
                        }
                    }
                    ids
                }
                _                                       => {
                    return (<P as Presenter<(), R>>::present_err(presenter, Error::BadRequest));
                }
            };
            presenter.try_present(T::append_links(&api::Entity::Id(id), &rel_ids).into_future().wait())
        }
        self.router.attach_append_many(T::resource_plural(), Rel::to_many(), append_many::<R, T, Rel, P, C>);
        self.router.attach_append_many_rel(T::resource_plural(), Rel::to_many(), append_many_rel::<R, T, Rel, P, C>);
    }

    pub fn attach_replace_many<T, Rel, P, C>(&mut self)
    where
        T: rel::raw::ReplaceMany<<P as Presenter<Rel::Resource, R>>::Include, Rel>,
        Rel: rel::ToMany,
        Rel::Resource: raw::RawUpdate + Deserialize,
        P: Presenter<Rel::Resource, R> + Presenter<(), R>,
        C: Receiver<Rel::Resource, Box<Read>>,
    {
        fn replace_many<R, T, Rel, P, C>(request: r::PostManyRequest, link_maker: R::LinkMaker) -> R::Response
        where
            R: Router,
            T: rel::raw::ReplaceMany<P::Include, Rel>,
            Rel: rel::ToMany,
            Rel::Resource: raw::RawUpdate + Deserialize,
            P: Presenter<Rel::Resource, R>,
            C: Receiver<Rel::Resource, Box<Read>>,
        {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(request.id.parse(), presenter);
            let received = try_status!(C::wrap(request.body).receive_collection(), presenter);
            presenter.try_present(T::replace_many(&api::Entity::Id(id), received).into_future().wait())
        }
        fn replace_many_rel<R, T, Rel, P, C>(request: r::UpdateRelRequest, link_maker: R::LinkMaker) -> R::Response
        where
            R: Router,
            T: rel::raw::ReplaceMany<<P as Presenter<Rel::Resource, R>>::Include, Rel>,
            Rel: rel::ToMany,
            Rel::Resource: raw::RawUpdate + Deserialize,
            P: Presenter<Rel::Resource, R> + Presenter<(), R>,
            C: Receiver<Rel::Resource, Box<Read>>,
        {
            let presenter = <P as Presenter<(), R>>::prepare(None, link_maker);
            let id = match request.id.parse() {
                Ok(id)  => id,
                Err(_)  => return (<P as Presenter<(), R>>::present_err(presenter, Error::Conflict)),
            };
            let rel = match C::wrap(request.body).receive_rel::<Rel>() {
                Ok(rel) => rel,
                Err(_)  => return (<P as Presenter<(), R>>::present_err(presenter, Error::Conflict)),
            };
            let rel_ids: Vec<_> = match rel {
                raw::Relationship::Many(identifiers)   => {
                    let mut ids = vec![];
                    for identifier in identifiers {
                        match identifier.id.parse() {
                            Ok(id)  => ids.push(id),
                            Err(_)  => return (<P as Presenter<(), R>>::present_err(presenter, Error::Conflict)),
                        }
                    }
                    ids
                }
                _                                       => {
                    return (<P as Presenter<(), R>>::present_err(presenter, Error::BadRequest));
                }
            };
            presenter.try_present(T::replace_links(&api::Entity::Id(id), &rel_ids).into_future().wait())
        }
        self.router.attach_replace_many(T::resource_plural(), Rel::to_many(), replace_many::<R, T, Rel, P, C>);
        self.router.attach_replace_many_rel(T::resource_plural(), Rel::to_many(), replace_many_rel::<R, T, Rel, P, C>);
    }

    pub fn attach_get_alias<T, P>(&mut self, route: &'static str)
    where
        T: raw::RawGetAliased<P::Include>,
        P: Presenter<T, R>,
    { 
        fn get_aliased<R, T, P>(alias_request: api::AliasRequest, get_request: r::GetRequest, link_maker: R::LinkMaker) -> R::Response
        where
            T: raw::RawGetAliased<P::Include>,
            P: Presenter<T, R>,
            R: Router,
        {
            let presenter = P::prepare(None, link_maker);
            presenter.try_present(T::get(alias_request, &get_request.includes).into_future().wait())
        }
        self.router.attach_get_alias(route, get_aliased::<R, T, P>);
    }
}
