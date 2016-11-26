#![allow(unused_parens)]

use api::{self, Error};
use api::async;
use api::rel;
use api::raw;
use router::{Request, Router, ResourceRoute, Method};
use futures::{IntoFuture, Future};
use receiver::{Receiver, IdReceiver, PatchReceiver};
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
        fn get<R, T, P>(request: R::Request, link_maker: R::LinkMaker) -> R::Response
        where
            T: raw::RawGet<P::Include>,
            P: Presenter<T, R>,
            R: Router,
        {
            let options = request.resource_options();
            let presenter = P::prepare(options.field_set, link_maker);
            let id = try_status!(request.id().parse(), presenter);
            presenter.try_present(T::get(id, options.includes).into_future().wait())
        }
        self.router.attach_resource(T::resource_plural(), ResourceRoute {
            method: Method::Get,
            relation: None,
        }, get::<R, T, P>);
    }

    pub fn attach_index<T, P>(&mut self)
    where
        T: raw::RawIndex<P::Include>,
        P: Presenter<T, R>,
    {
        fn index<R, T, P>(request: R::Request, link_maker: R::LinkMaker) -> R::Response
        where 
            R: Router,
            T: raw::RawIndex<P::Include>,
            P: Presenter<T, R>,
        {
            let options = request.collection_options();
            let presenter = P::prepare(options.field_set, link_maker);
            presenter.try_present(T::index(options.includes, options.sort, options.page).into_future().wait())
        }
        self.router.attach_resource(T::resource_plural(), ResourceRoute {
            method: Method::Index,
            relation: None,
        }, index::<R, T, P>);
    }

    pub fn attach_delete<T, P>(&mut self)
    where
        T: api::Delete,
        P: Presenter<(), R>,
    {
        fn delete<R, T, P>(request: R::Request, link_maker: R::LinkMaker) -> R::Response
        where
            T: api::Delete,
            P: Presenter<(), R>,
            R: Router,
        {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(request.id().parse(), presenter);
            presenter.try_present(T::delete(&id).into_future().wait())
        }
        self.router.attach_resource(T::resource_plural(), ResourceRoute {
            method: Method::Delete,
            relation: None,
        }, delete::<R, T, P>);
    }

    pub fn attach_clear<T, P>(&mut self)
    where
        T: api::Clear,
        P: Presenter<(), R>,
    {
        fn clear<R, T, P>(_: R::Request, link_maker: R::LinkMaker) -> R::Response
        where
            T: api::Clear,
            P: Presenter<(), R>,
            R: Router,
        {
            let presenter = P::prepare(None, link_maker);
            presenter.try_present(T::clear().into_future().wait())
        }
        self.router.attach_resource(T::resource_plural(), ResourceRoute {
            method: Method::Clear,
            relation: None,
        }, clear::<R, T, P>);
    }

    pub fn attach_remove<T, P, C>(&mut self)
    where
        T: api::Remove,
        P: Presenter<(), R>,
        C: IdReceiver<T, R::Request>,
    {
        fn remove<R, T, P, C>(request: R::Request, link_maker: R::LinkMaker) -> R::Response
        where
            T: api::Remove,
            P: Presenter<(), R>,
            R: Router,
            C: IdReceiver<T, R::Request>,
        {
            let presenter = P::prepare(None, link_maker);
            let ids = try_status!(C::receive_ids(request), presenter);
            presenter.try_present(T::remove(&ids).into_future().wait())
        }
        self.router.attach_resource(T::resource_plural(), ResourceRoute {
            method: Method::Remove,
            relation: None,
        }, remove::<R, T, P, C>);
    }

    pub fn attach_patch<T, P, C>(&mut self)
    where
        T: raw::RawPatch<P::Include>,
        P: Presenter<T, R>,
        C: PatchReceiver<T, R::Request, raw::Synchronous>,
    {
        fn patch<R, T, P, C>(request: R::Request, link_maker: R::LinkMaker) -> R::Response
        where
            R: Router,
            T: raw::RawPatch<P::Include>,
            P: Presenter<T, R>,
            C: PatchReceiver<T, R::Request, raw::Synchronous>,
        {
            let options = request.resource_options();
            let presenter = P::prepare(options.field_set, link_maker);
            let id = try_status!(request.id().parse(), presenter);
            let received = try_status!(C::receive_patch(request), presenter);
            presenter.try_present(T::patch(id, received).into_future().wait())
        }
        self.router.attach_resource(T::resource_plural(), ResourceRoute {
            method: Method::Patch,
            relation: None,
        }, patch::<R, T, P, C>);
    }

    pub fn attach_patch_async<T, P, C>(&mut self)
    where
        T: async::raw::RawPatchAsync,
        P: Presenter<T::Job, R>,
        C: PatchReceiver<T, R::Request, async::raw::Asynchronous>,
    {
        fn patch_async<R, T, P, C>(request: R::Request, link_maker: R::LinkMaker) -> R::Response
        where
            T: async::raw::RawPatchAsync,
            P: Presenter<T::Job, R>,
            R: Router,
            C: PatchReceiver<T, R::Request, async::raw::Asynchronous>,
        {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(request.id().parse(), presenter);
            let received = try_status!(C::receive_patch(request), presenter);
            presenter.try_present(T::patch_async(id, received).into_future().wait())
        }
        self.router.attach_resource(T::resource_plural(), ResourceRoute {
            method: Method::Patch,
            relation: None,
        }, patch_async::<R, T, P, C>);
    }

    pub fn attach_post<T, P, C>(&mut self)
    where
        T: raw::RawPost<P::Include>,
        P: Presenter<T, R>,
        C: Receiver<T, R::Request>,
    {
        fn post<R, T, P, C>(request: R::Request, link_maker: R::LinkMaker) -> R::Response
        where
            R: Router,
            T: raw::RawPost<P::Include>,
            P: Presenter<T, R>,
            C: Receiver<T, R::Request>,
        {
            let options = request.resource_options();
            let presenter = P::prepare(options.field_set, link_maker);
            let received = try_status!(C::receive_resource(request), presenter);
            presenter.try_present(T::post(received).into_future().wait())
        }
        self.router.attach_resource(T::resource_plural(), ResourceRoute {
            method: Method::Post,
            relation: None,
        }, post::<R, T, P, C>);
    }

    pub fn attach_post_async<T, P, C>(&mut self)
    where
        T: async::raw::RawPostAsync,
        P: Presenter<T::Job, R>,
        C: Receiver<T, R::Request>,
    {
        fn post_async<R, T, P, C>(request: R::Request, link_maker: R::LinkMaker) -> R::Response
        where
            R: Router,
            T: async::raw::RawPostAsync,
            P: Presenter<T::Job, R>,
            C: Receiver<T, R::Request>,
        {
            let presenter = P::prepare(None, link_maker);
            let received = try_status!(C::receive_resource(request), presenter);
            presenter.try_present(T::post_async(received).into_future().wait())
        }
        self.router.attach_resource(T::resource_plural(), ResourceRoute {
            method: Method::Post,
            relation: None,
        }, post_async::<R, T, P, C>);
    }

    pub fn attach_append<T, P, C>(&mut self)
    where
        T: raw::RawAppend<P::Include>,
        P: Presenter<T, R>,
        C: Receiver<T, R::Request>,
    {
        fn append<R, T, P, C>(request: R::Request, link_maker: R::LinkMaker) -> R::Response
        where
            R: Router,
            T: raw::RawAppend<P::Include>,
            P: Presenter<T, R>,
            C: Receiver<T, R::Request>,
        {
            let options = request.collection_options();
            let presenter = P::prepare(options.field_set, link_maker);
            let received = try_status!(C::receive_collection(request), presenter);
            presenter.try_present(T::append(received).into_future().wait())
        }
        self.router.attach_resource(T::resource_plural(), ResourceRoute {
            method: Method::Append,
            relation: None,
        }, append::<R, T, P, C>);
    }

    pub fn attach_replace<T, P, C>(&mut self)
    where
        T: raw::RawReplace<P::Include>,
        P: Presenter<T, R>,
        C: Receiver<T, R::Request>,
    {
        fn replace<R, T, P, C>(request: R::Request, link_maker: R::LinkMaker) -> R::Response
        where
            T: raw::RawReplace<P::Include>,
            P: Presenter<T, R>,
            R: Router,
            C: Receiver<T, R::Request>,
        {
            let options = request.collection_options();
            let presenter = P::prepare(options.field_set, link_maker);
            let received = try_status!(C::receive_collection(request), presenter);
            presenter.try_present(T::replace(received).into_future().wait())
        }
        self.router.attach_resource(T::resource_plural(), ResourceRoute {
            method: Method::Replace,
            relation: None,
        }, replace::<R, T, P, C>);
    }

    pub fn attach_fetch_one<T, Rel, P>(&mut self)
    where
        T: rel::raw::GetOne<P::Include, Rel>,
        Rel: rel::ToOne,
        Rel::Resource: raw::RawFetch,
        P: Presenter<Rel::Resource, R>,
    {
        fn fetch_one<R, T, Rel, P>(request: R::Request, link_maker: R::LinkMaker) -> R::Response
        where
            T: rel::raw::GetOne<P::Include, Rel>,
            Rel: rel::ToOne,
            Rel::Resource: raw::RawFetch,
            P: Presenter<Rel::Resource, R>,
            R: Router,
        {
            let options = request.resource_options();
            let presenter = P::prepare(options.field_set, link_maker);
            let id = try_status!(request.id().parse(), presenter);
            presenter.try_present(T::get_one(&api::Entity::Id(id), options.includes).into_future().wait())
        }
        fn fetch_one_rel<R, T, Rel, P>(request: R::Request, link_maker: R::LinkMaker) -> R::Response
        where
            T: rel::raw::GetOne<P::Include, Rel>,
            Rel: rel::ToOne,
            Rel::Resource: raw::RawFetch,
            P: Presenter<Rel::Resource, R>,
            R: Router,
        {
            let options = request.resource_options();
            let presenter = P::prepare(options.field_set, link_maker);
            let parsed_id = try_status!(request.id().parse(), presenter);
            presenter.try_present(T::has_one(&api::Entity::Id(parsed_id)).into_future().wait().map(move |rel| {
                raw::RelResponse {
                    resource: T::resource_plural(),
                    related: Rel::to_one(),
                    id: request.id().to_owned(),
                    rel: raw::Relationship::One(rel.map(|id| raw::Identifier::new::<Rel::Resource>(&id))),
                    includes: vec![],
                }
            }))
        }
        self.router.attach_resource(T::resource_plural(), ResourceRoute {
            method: Method::Get,
            relation: Some((Rel::to_one(), false)),
        }, fetch_one::<R, T, Rel, P>);
        self.router.attach_resource(T::resource_plural(), ResourceRoute {
            method: Method::Get,
            relation: Some((Rel::to_one(), true))
        }, fetch_one_rel::<R, T, Rel, P>);
    }

    pub fn attach_fetch_many<T, Rel, P>(&mut self)
    where
        T: rel::raw::IndexMany<P::Include, Rel>,
        Rel: rel::ToMany,
        Rel::Resource: raw::RawFetch,
        P: Presenter<Rel::Resource, R>,
    {
        fn fetch_many<R, T, Rel, P>(request: R::Request, link_maker: R::LinkMaker) -> R::Response
        where
            T: rel::raw::IndexMany<P::Include, Rel>,
            Rel: rel::ToMany,
            Rel::Resource: raw::RawFetch,
            P: Presenter<Rel::Resource, R>,
            R: Router,
        {
            let options = request.collection_options();
            let presenter = P::prepare(options.field_set, link_maker);
            let id = try_status!(request.id().parse(), presenter);
            presenter.try_present(T::index_many(&api::Entity::Id(id), options.includes).into_future().wait())
        }
        fn fetch_many_rel<R, T, Rel, P>(request: R::Request, link_maker: R::LinkMaker) -> R::Response
        where 
            T: rel::raw::IndexMany<P::Include, Rel>,
            Rel: rel::ToMany,
            Rel::Resource: raw::RawFetch,
            P: Presenter<Rel::Resource, R>,
            R: Router,
        {
            let options = request.collection_options();
            let presenter = P::prepare(options.field_set, link_maker);
            let parsed_id = try_status!(request.id().parse(), presenter);
            presenter.try_present(T::has_many(&api::Entity::Id(parsed_id)).into_future().wait().map(move |rel| {
                raw::RelResponse {
                    resource: T::resource_plural(),
                    related: Rel::to_many(),
                    id: request.id().to_owned(),
                    rel: raw::Relationship::Many(rel.into_iter().map(|id| raw::Identifier::new::<Rel::Resource>(&id)).collect()),
                    includes: vec![],
                }
            }))
        }
        self.router.attach_resource(T::resource_plural(), ResourceRoute {
            method: Method::Index,
            relation: Some((Rel::to_many(), false))
        }, fetch_many::<R, T, Rel, P>);
        self.router.attach_resource(T::resource_plural(), ResourceRoute {
            method: Method::Index,
            relation: Some((Rel::to_many(), true))
        }, fetch_many_rel::<R, T, Rel, P>);
    }

    pub fn attach_delete_one<T, Rel, P>(&mut self)
    where
        T: rel::raw::DeleteOne<Rel>,
        Rel: rel::ToOne,
        P: Presenter<(), R>,
    {
        fn delete_one<R, T, Rel, P>(request: R::Request, link_maker: R::LinkMaker) -> R::Response
        where
            T: rel::raw::DeleteOne<Rel>,
            Rel: rel::ToOne,
            P: Presenter<(), R>,
            R: Router,
        {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(request.id().parse(), presenter);
            presenter.try_present(T::delete_one(&api::Entity::Id(id)).into_future().wait())
        }
        fn delete_one_rel<R, T, Rel, P>(request: R::Request, link_maker: R::LinkMaker) -> R::Response
        where
            T: rel::raw::DeleteOne<Rel>,
            Rel: rel::ToOne,
            P: Presenter<(), R>,
            R: Router,
        {
            let presenter = P::prepare(None, link_maker);
            let parsed_id = try_status!(request.id().parse(), presenter);
            presenter.try_present(T::unlink_one(&api::Entity::Id(parsed_id)).into_future().wait().map(|_| ()))
        }
        self.router.attach_resource(T::resource_plural(), ResourceRoute {
            method: Method::Delete,
            relation: Some((Rel::to_one(), false))
        }, delete_one::<R, T, Rel, P>);
        self.router.attach_resource(T::resource_plural(), ResourceRoute {
            method: Method::Delete,
            relation: Some((Rel::to_one(), true))
        }, delete_one_rel::<R, T, Rel, P>)
    }

    pub fn attach_clear_many<T, Rel, P>(&mut self)
    where
        T: rel::raw::ClearMany<Rel>,
        Rel: rel::ToMany,
        P: Presenter<(), R>,
    {
        fn clear_many<R, T, Rel, P>(request: R::Request, link_maker: R::LinkMaker) -> R::Response
        where
            T: rel::raw::ClearMany<Rel>,
            Rel: rel::ToMany,
            P: Presenter<(), R>,
            R: Router,
        {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(request.id().parse(), presenter);
            presenter.try_present(T::clear_many(&api::Entity::Id(id)).into_future().wait())
        }
        fn clear_many_rel<R, T, Rel, P>(request: R::Request, link_maker: R::LinkMaker) -> R::Response
        where
            T: rel::raw::ClearMany<Rel>,
            Rel: rel::ToMany,
            P: Presenter<(), R>,
            R: Router,
        {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(request.id().parse(), presenter);
            presenter.try_present(T::clear_links(&api::Entity::Id(id)).into_future().wait().map(|_| ()))
        }
        self.router.attach_resource(T::resource_plural(), ResourceRoute {
            method: Method::Clear,
            relation: Some((Rel::to_many(), true)),
        }, clear_many::<R, T, Rel, P>);
        self.router.attach_resource(T::resource_plural(), ResourceRoute {
            method: Method::Clear,
            relation: Some((Rel::to_many(), false)),
        }, clear_many_rel::<R, T, Rel, P>);
    }

    pub fn attach_remove_many<T, Rel, P, C>(&mut self)
    where
        T: rel::raw::RemoveMany<Rel>,
        Rel: rel::ToMany,
        P: Presenter<(), R>,
        C: IdReceiver<Rel::Resource, R::Request>,
    {
        fn remove_many<R, T, Rel, P, C>(request: R::Request, link_maker: R::LinkMaker) -> R::Response
        where
            R: Router,
            T: rel::raw::RemoveMany<Rel>,
            Rel: rel::ToMany,
            P: Presenter<(), R>,
            C: IdReceiver<Rel::Resource, R::Request>,
        {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(request.id().parse(), presenter);
            let parsed_rel_ids = try_status!(C::receive_ids(request), presenter);
            presenter.try_present(T::remove_many(&api::Entity::Id(id), &parsed_rel_ids).into_future().wait())
        }
        fn remove_many_rel<R, T, Rel, P, C>(request: R::Request, link_maker: R::LinkMaker) -> R::Response
        where
            R: Router,
            T: rel::raw::RemoveMany<Rel>,
            Rel: rel::ToMany,
            P: Presenter<(), R>,
            C: IdReceiver<Rel::Resource, R::Request>,
        {
            let presenter = P::prepare(None, link_maker);
            let parsed_id = try_status!(request.id().parse(), presenter);
            let parsed_rel_ids = try_status!(C::receive_ids(request), presenter);
            presenter.try_present(T::remove_links(&api::Entity::Id(parsed_id), &parsed_rel_ids).into_future().wait().map(|_| ()))
        }
        self.router.attach_resource(T::resource_plural(), ResourceRoute {
            method: Method::Remove,
            relation: Some((Rel::to_many(), false))
        }, remove_many::<R, T, Rel, P, C>);
        self.router.attach_resource(T::resource_plural(), ResourceRoute {
            method: Method::Remove,
            relation: Some((Rel::to_many(), true))
        }, remove_many_rel::<R, T, Rel, P, C>);
    }

    pub fn attach_patch_one<T, Rel, P, C>(&mut self)
    where
        T: rel::raw::PatchOne<P::Include, Rel>,
        Rel: rel::ToOne,
        Rel::Resource: raw::RawHasPatch<raw::Synchronous>,
        P: Presenter<Rel::Resource, R>,
        C: PatchReceiver<Rel::Resource, R::Request, raw::Synchronous>,
    {
        fn patch_one<R, T, Rel, P, C>(request: R::Request, link_maker: R::LinkMaker) -> R::Response
        where
            R: Router,
            T: rel::raw::PatchOne<P::Include, Rel>,
            Rel: rel::ToOne,
            Rel::Resource: raw::RawHasPatch<raw::Synchronous>,
            P: Presenter<Rel::Resource, R>,
            C: PatchReceiver<Rel::Resource, R::Request, raw::Synchronous>,
        {
            let options = request.resource_options();
            let presenter = P::prepare(options.field_set, link_maker);
            let id = try_status!(request.id().parse(), presenter);
            let received = try_status!(C::receive_patch(request), presenter);
            presenter.try_present(T::patch_one(&api::Entity::Id(id), received).into_future().wait())
        }
        self.router.attach_resource(T::resource_plural(), ResourceRoute {
            method: Method::Patch,
            relation: Some((Rel::to_one(), false))
        }, patch_one::<R, T, Rel, P, C>);
    }

    pub fn attach_post_one<T, Rel, P, C>(&mut self)
    where
        T: rel::raw::PostOne<<P as Presenter<Rel::Resource, R>>::Include, Rel>,
        Rel: rel::ToOne,
        Rel::Resource: raw::RawUpdate,
        P: Presenter<Rel::Resource, R> + Presenter<(), R>,
        C: Receiver<Rel::Resource, R::Request>,
    {
        fn post_one<R, T, Rel, P, C>(request: R::Request, link_maker: R::LinkMaker) -> R::Response
        where
            R: Router,
            T: rel::raw::PostOne<P::Include, Rel>,
            Rel: rel::ToOne,
            Rel::Resource: raw::RawUpdate,
            P: Presenter<Rel::Resource, R>,
            C: Receiver<Rel::Resource, R::Request>,
        {
            let options = request.resource_options();
            let presenter = P::prepare(options.field_set, link_maker);
            let id = try_status!(request.id().parse(), presenter);
            let received = try_status!(C::receive_resource(request), presenter);
            presenter.try_present(T::post_one(api::Entity::Id(id), received).into_future().wait())
        }
        fn post_one_rel<R, T, Rel, P, C>(request: R::Request, link_maker: R::LinkMaker) -> R::Response
        where
            R: Router,
            T: rel::raw::PostOne<<P as Presenter<Rel::Resource, R>>::Include, Rel>,
            Rel: rel::ToOne,
            Rel::Resource: raw::RawUpdate,
            P: Presenter<Rel::Resource, R> + Presenter<(), R>,
            C: Receiver<Rel::Resource, R::Request>,
        {
            let options = request.resource_options();
            let presenter = <P as Presenter<(), R>>::prepare(options.field_set, link_maker);
            let id = match request.id().parse() {
                Ok(id)  => id,
                Err(_)  => return (<P as Presenter<(), R>>::present_err(presenter, Error::Conflict)),
            };
            let identifier = match C::receive_to_one::<Rel>(request) {
                Ok(rel) => rel,
                Err(_)  => return (<P as Presenter<(), R>>::present_err(presenter, Error::Conflict)),
            };
            match identifier {
                Some(identifier)    => {
                    let rel_id = match identifier.id.parse() {
                        Ok(id)  => id,
                        Err(_)  => return (<P as Presenter<(), R>>::present_err(presenter, Error::Conflict)),
                    };
                    presenter.try_present(T::link_one(&api::Entity::Id(id), &rel_id).into_future().wait())
                }
                None                => {
                    presenter.try_present(T::unlink_one(&api::Entity::Id(id)).into_future().wait().map(|_| ()))
                }
            }
        }
        self.router.attach_resource(T::resource_plural(), ResourceRoute {
            method: Method::Post,
            relation: Some((Rel::to_one(), false))
        }, post_one::<R, T, Rel, P, C>);
        self.router.attach_resource(T::resource_plural(), ResourceRoute {
            method: Method::Post,
            relation: Some((Rel::to_one(), true))
        }, post_one_rel::<R, T, Rel, P, C>);
    }

    pub fn attach_append_many<T, Rel, P, C>(&mut self)
    where
        T: rel::raw::AppendMany<<P as Presenter<Rel::Resource, R>>::Include, Rel>,
        Rel: rel::ToMany,
        Rel::Resource: raw::RawUpdate,
        P: Presenter<Rel::Resource, R> + Presenter<(), R>,
        C: Receiver<Rel::Resource, R::Request>,
    {
        fn append_many<R, T, Rel, P, C>(request: R::Request, link_maker: R::LinkMaker) -> R::Response
        where
            R: Router,
            T: rel::raw::AppendMany<P::Include, Rel>,
            Rel: rel::ToMany,
            Rel::Resource: raw::RawUpdate,
            P: Presenter<Rel::Resource, R>,
            C: Receiver<Rel::Resource, R::Request>,
        {
            let options = request.collection_options();
            let presenter = P::prepare(options.field_set, link_maker);
            let id = try_status!(request.id().parse(), presenter);
            let received = try_status!(C::receive_collection(request), presenter);
            presenter.try_present(T::append_many(api::Entity::Id(id), received).into_future().wait())
        }
        fn append_many_rel<R, T, Rel, P, C>(request: R::Request, link_maker: R::LinkMaker) -> R::Response
        where
            R: Router,
            T: rel::raw::AppendMany<<P as Presenter<Rel::Resource, R>>::Include, Rel>,
            Rel: rel::ToMany,
            Rel::Resource: raw::RawUpdate,
            P: Presenter<Rel::Resource, R> + Presenter<(), R>,
            C: Receiver<Rel::Resource, R::Request>,
        {
            let presenter = <P as Presenter<(), R>>::prepare(None, link_maker);
            let id = match request.id().parse() {
                Ok(id)  => id,
                Err(_)  => return (<P as Presenter<(), R>>::present_err(presenter, Error::Conflict)),
            };
            let identifiers = match C::receive_to_many::<Rel>(request) {
                Ok(rel) => rel,
                Err(_)  => return (<P as Presenter<(), R>>::present_err(presenter, Error::Conflict)),
            };
            let rel_ids: Vec<_> = {
                let mut ids = vec![];
                for identifier in identifiers {
                    match identifier.id.parse() {
                        Ok(id)  => ids.push(id),
                        Err(_)  => return (<P as Presenter<(), R>>::present_err(presenter, Error::Conflict)),
                    }
                }
                ids
            };
            presenter.try_present(T::append_links(&api::Entity::Id(id), &rel_ids).into_future().wait())
        }
        self.router.attach_resource(T::resource_plural(), ResourceRoute {
            method: Method::Append,
            relation: Some((Rel::to_many(), false))
        }, append_many::<R, T, Rel, P, C>);
        self.router.attach_resource(T::resource_plural(), ResourceRoute {
            method: Method::Append,
            relation: Some((Rel::to_many(), true))
        }, append_many_rel::<R, T, Rel, P, C>);
    }

    pub fn attach_replace_many<T, Rel, P, C>(&mut self)
    where
        T: rel::raw::ReplaceMany<<P as Presenter<Rel::Resource, R>>::Include, Rel>,
        Rel: rel::ToMany,
        Rel::Resource: raw::RawUpdate,
        P: Presenter<Rel::Resource, R> + Presenter<(), R>,
        C: Receiver<Rel::Resource, R::Request>,
    {
        fn replace_many<R, T, Rel, P, C>(request: R::Request, link_maker: R::LinkMaker) -> R::Response
        where
            R: Router,
            T: rel::raw::ReplaceMany<P::Include, Rel>,
            Rel: rel::ToMany,
            Rel::Resource: raw::RawUpdate,
            P: Presenter<Rel::Resource, R>,
            C: Receiver<Rel::Resource, R::Request>,
        {
            let options = request.collection_options();
            let presenter = P::prepare(options.field_set, link_maker);
            let id = try_status!(request.id().parse(), presenter);
            let received = try_status!(C::receive_collection(request), presenter);
            presenter.try_present(T::replace_many(api::Entity::Id(id), received).into_future().wait())
        }
        fn replace_many_rel<R, T, Rel, P, C>(request: R::Request, link_maker: R::LinkMaker) -> R::Response
        where
            R: Router,
            T: rel::raw::ReplaceMany<<P as Presenter<Rel::Resource, R>>::Include, Rel>,
            Rel: rel::ToMany,
            Rel::Resource: raw::RawUpdate,
            P: Presenter<Rel::Resource, R> + Presenter<(), R>,
            C: Receiver<Rel::Resource, R::Request>,
        {
            let options = request.resource_options();
            let presenter = <P as Presenter<(), R>>::prepare(options.field_set, link_maker);
            let id = match request.id().parse() {
                Ok(id)  => id,
                Err(_)  => return (<P as Presenter<(), R>>::present_err(presenter, Error::Conflict)),
            };
            let identifiers = match C::receive_to_many::<Rel>(request) {
                Ok(rel) => rel,
                Err(_)  => return (<P as Presenter<(), R>>::present_err(presenter, Error::Conflict)),
            };
            let rel_ids: Vec<_> = {
                let mut ids = vec![];
                for identifier in identifiers {
                    match identifier.id.parse() {
                        Ok(id)  => ids.push(id),
                        Err(_)  => return (<P as Presenter<(), R>>::present_err(presenter, Error::Conflict)),
                    }
                }
                ids
            };
            presenter.try_present(T::replace_links(&api::Entity::Id(id), &rel_ids).into_future().wait())
        }
        self.router.attach_resource(T::resource_plural(), ResourceRoute {
            method: Method::Replace,
            relation: Some((Rel::to_many(), false))
        }, replace_many::<R, T, Rel, P, C>);
        self.router.attach_resource(T::resource_plural(), ResourceRoute {
            method: Method::Replace,
            relation: Some((Rel::to_many(), true))
        }, replace_many_rel::<R, T, Rel, P, C>);
    }

    pub fn attach_get_alias<T, P>(&mut self, route: &'static str)
    where
        T: raw::RawGetAliased<P::Include>,
        P: Presenter<T, R>,
    { 
        fn get_aliased<R, T, P>(request: R::Request, link_maker: R::LinkMaker) -> R::Response
        where
            T: raw::RawGetAliased<P::Include>,
            P: Presenter<T, R>,
            R: Router,
        {
            let options = request.resource_options();
            let presenter = P::prepare(options.field_set, link_maker);
            presenter.try_present(T::get(request.alias_info(), options.includes).into_future().wait())
        }
        self.router.attach_alias(route, Method::Get, get_aliased::<R, T, P>);
    }

    pub fn attach_index_aliased<T, P>(&mut self, route: &'static str)
    where
        T: raw::RawIndexAliased<P::Include>,
        P: Presenter<T, R>,
    {
        fn index_aliased<R, T, P>(request: R::Request, link_maker: R::LinkMaker) -> R::Response
        where
            T: raw::RawIndexAliased<P::Include>,
            P: Presenter<T, R>,
            R: Router,
        {
            let options = request.collection_options();
            let presenter = P::prepare(options.field_set, link_maker);
            presenter.try_present(T::index(request.alias_info(), options.includes, options.sort).into_future().wait())
        }
        self.router.attach_alias(route, Method::Index, index_aliased::<R, T, P>);
    }
}
