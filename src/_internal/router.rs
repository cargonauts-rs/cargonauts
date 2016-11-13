use json;

use api::{self, Error};
use api::rel;
use api::raw;
use router::Router as RouterTrait;
use Deserialize;
use futures::{IntoFuture, Future};
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

pub struct Router<'a, R: RouterTrait + 'a> {
    router: &'a mut R,
}

impl<'a, R: RouterTrait> Router<'a, R> {
    pub fn new(router: &'a mut R) -> Router<'a, R> {
        Router {
            router: router,
        }
    }

    pub fn attach_get<T, P>(&mut self)
    where
        T: raw::RawGet<P::Include>,
        P: Presenter<T, R>,
    {
        self.router.attach_get(T::resource_plural(), |request, link_maker| {
            let presenter = P::prepare(request.field_set, link_maker);
            let id = try_status!(request.id.parse(), presenter);
            presenter.try_present(T::get_id(id, &request.includes).into_future().wait())
        });
    }

    pub fn attach_index<T, P>(&mut self)
    where
        T: raw::RawIndex<P::Include>,
        P: Presenter<T, R>,
    {
        self.router.attach_index(T::resource_plural(), |request, link_maker| {
            let presenter = P::prepare(request.field_set, link_maker);
            presenter.try_present(T::index(&request.includes, &request.sort, &request.page).into_future().wait())
        });
    }

    pub fn attach_patch<T, P>(&mut self)
    where
        T: raw::RawPatch<P::Include>,
        P: Presenter<T, R>,
    {
        self.router.attach_patch(T::resource_plural(), |request, link_maker| {
            let presenter = P::prepare(request.field_set, link_maker);
            let id = try_status!(request.id.parse(), presenter);
            let patch = try_status!(json::from_reader(request.attributes), presenter);
            let rels = try_status!(<<T as raw::RawUpdate>::Relationships as raw::UpdateRelationships>::from_iter(request.relationships.into_iter()), presenter);
            presenter.try_present(T::patch(id, patch, rels).into_future().wait())
        });
    }

    pub fn attach_patch_async<T, P>(&mut self)
    where
        T: raw::RawPatchAsync,
        P: Presenter<T::Job, R>,
    {
        self.router.attach_patch(T::resource_plural(), |request, link_maker| {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(request.id.parse(), presenter);
            let patch = try_status!(json::from_reader(request.attributes), presenter);
            let rels = try_status!(<<T as raw::RawUpdate>::Relationships as raw::UpdateRelationships>::from_iter(request.relationships.into_iter()), presenter);
            presenter.try_present(T::patch_async(id, patch, rels).into_future().wait())
        });
    }

    pub fn attach_post<T, P>(&mut self)
    where
        T: raw::RawPost<P::Include>,
        P: Presenter<T, R>,
    {
        self.router.attach_post(T::resource_plural(), |request, link_maker| {
            let presenter = P::prepare(request.field_set, link_maker);
            let post = try_status!(json::from_reader(request.attributes), presenter);
            let rels = try_status!(<<T as raw::RawUpdate>::Relationships as raw::UpdateRelationships>::from_iter(request.relationships.into_iter()), presenter);
            presenter.try_present(T::post(post, rels).into_future().wait())
        });
    }

    pub fn attach_post_async<T, P>(&mut self)
    where
        T: raw::RawPostAsync,
        P: Presenter<T::Job, R>,
    {
        self.router.attach_post(T::resource_plural(), |request, link_maker| {
            let presenter = P::prepare(None, link_maker);
            let post = try_status!(json::from_reader(request.attributes), presenter);
            let rels = try_status!(<<T as raw::RawUpdate>::Relationships as raw::UpdateRelationships>::from_iter(request.relationships.into_iter()), presenter);
            presenter.try_present(T::post_async(post, rels).into_future().wait())
        });
    }

    pub fn attach_delete<T, P>(&mut self)
    where
        T: api::Delete,
        P: Presenter<(), R>,
    {
        self.router.attach_delete(T::resource_plural(), |id, link_maker| {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(id.parse(), presenter);
            // TODO definitely wrong
            match T::delete(&id).into_future().wait() {
                Ok(_)       => presenter.present_err(Error::NoContent),
                Err(error)  => presenter.present_err(error.into()),
            }
        });
    }

    pub fn attach_get_alias<T, P>(&mut self, route: &'static str)
    where
        T: raw::RawGetAliased<P::Include>,
        P: Presenter<T, R>,
    { 
        self.router.attach_get_alias(route, |alias_request, get_request, link_maker| {
            let presenter = P::prepare(None, link_maker);
            presenter.try_present(T::get(alias_request, &get_request.includes).into_future().wait())
        });
    }

    pub fn attach_fetch_one<T, Rel, P>(&mut self)
    where
        T: rel::raw::FetchOne<P::Include, Rel>,
        Rel: rel::Relation,
        Rel::Resource: raw::RawFetch,
        P: Presenter<Rel::Resource, R>,
    {
        self.router.attach_fetch_one(T::resource_plural(), Rel::to_one(), |request, link_maker| {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(request.id.parse(), presenter);
            presenter.try_present(T::fetch_one(&api::Entity::Id(id), &request.includes).into_future().wait())
        });
        self.router.attach_fetch_rel(T::resource_plural(), Rel::to_one(), |request, link_maker| {
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
        });
    }

    pub fn attach_fetch_many<T, Rel, P>(&mut self)
    where
        T: rel::raw::FetchMany<P::Include, Rel>,
        Rel: rel::Relation,
        Rel::Resource: raw::RawFetch,
        P: Presenter<Rel::Resource, R>,
    {
        self.router.attach_fetch_many(T::resource_plural(), Rel::to_many(), |request, link_maker| {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(request.id.parse(), presenter);
            presenter.try_present(T::fetch_many(&api::Entity::Id(id), &request.includes).into_future().wait())
        });
        self.router.attach_fetch_rel(T::resource_plural(), Rel::to_many(), |request, link_maker| {
            let presenter = P::prepare(None, link_maker);
            let parsed_id = try_status!(request.id.parse(), presenter);
            presenter.try_present(T::has_many(&api::Entity::Id(parsed_id)).into_future().wait().map(move |rel| {
                raw::RelResponse {
                    resource: T::resource_plural(),
                    related: Rel::to_one(),
                    id: request.id,
                    rel: raw::Relationship::Many(rel.into_iter().map(|id| raw::Identifier::new::<Rel::Resource>(&id)).collect()),
                    includes: vec![],
                }
            }))
        });
    }

    pub fn attach_delete_on<T, Rel, P>(&mut self)
    where
        T: rel::raw::DeleteOne<Rel>,
        Rel: rel::Relation,
        P: Presenter<(), R>
    {
        self.router.attach_delete_one(T::resource_plural(), Rel::to_one(), |id, link_maker| {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(id.parse(), presenter);
            // TODO definitely wrong
            match T::delete_one(&api::Entity::Id(id)).into_future().wait() {
                Ok(_)       => presenter.present_err(Error::NoContent),
                Err(error)  => presenter.present_err(error),
            }
        });
        self.router.attach_delete_one_rel(T::resource_plural(), Rel::to_one(), |id, link_maker| {
            let presenter = P::prepare(None, link_maker);
            let parsed_id = try_status!(id.parse(), presenter);
            match T::unlink_one(&api::Entity::Id(parsed_id)).into_future().wait() {
                Ok(_)       => presenter.present_err(Error::NoContent),
                Err(error)  => presenter.present_err(error),
            }
        });
    }

    pub fn attach_clear_many<T, Rel, P>(&mut self)
    where
        T: rel::raw::Clear<Rel>,
        Rel: rel::Relation,
        P: Presenter<(), R>
    {
        self.router.attach_clear_many(T::resource_plural(), Rel::to_many(), |id, link_maker| {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(id.parse(), presenter);
            // TODO definitely wrong
            match T::clear_many(&api::Entity::Id(id)).into_future().wait() {
                Ok(_)       => presenter.present_err(Error::NoContent),
                Err(error)  => presenter.present_err(error),
            }
        });
        self.router.attach_clear_many_rel(T::resource_plural(), Rel::to_many(), |id, link_maker| {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(id.parse(), presenter);
            // TODO definitely wrong
            match T::clear_links(&api::Entity::Id(id)).into_future().wait() {
                Ok(_)       => presenter.present_err(Error::NoContent),
                Err(error)  => presenter.present_err(error),
            }
        });
    }

    pub fn attach_remove_many<T, Rel, P>(&mut self)
    where
        T: rel::raw::Remove<Rel>,
        Rel: rel::Relation,
        P: Presenter<(), R>
    {
        self.router.attach_remove_many(T::resource_plural(), Rel::to_many(), |id, rel_ids, link_maker| {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(id.parse(), presenter);
            let parsed_rel_ids = try_status!(rel_ids.iter().map(|id| id.parse()).collect::<Result<Vec<_>, _>>(), presenter);
            // TODO definitely wrong
            match T::remove_many(&api::Entity::Id(id), &parsed_rel_ids).into_future().wait() {
                Ok(_)       => presenter.present_err(Error::NoContent),
                Err(error)  => presenter.present_err(error),
            }
        });
        self.router.attach_remove_many_rel(T::resource_plural(), Rel::to_many(), |id, rel_ids, link_maker| {
            let presenter = P::prepare(None, link_maker);
            let parsed_id = try_status!(id.parse(), presenter);
            let parsed_rel_ids = try_status!(rel_ids.iter().map(|id| id.parse()).collect::<Result<Vec<_>, _>>(), presenter);
            // TODO definitely wrong
            match T::remove_links(&api::Entity::Id(parsed_id), &parsed_rel_ids).into_future().wait() {
                Ok(_)       => presenter.present_err(Error::NoContent),
                Err(error)  => presenter.present_err(error),
            }
        });
    }

    pub fn attach_patch_one<T, Rel, P>(&mut self)
    where
        T: rel::raw::PatchOne<P::Include, Rel>,
        Rel: rel::Relation,
        Rel::Resource: raw::RawUpdate,
        P: Presenter<Rel::Resource, R>,
    {
        self.router.attach_patch_one(T::resource_plural(), Rel::to_one(), |request, link_maker| {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(request.id.parse(), presenter);
            let patch = try_status!(json::from_reader(request.attributes), presenter);
            let rels = try_status!(<<Rel::Resource as raw::RawUpdate>::Relationships as raw::UpdateRelationships>::from_iter(request.relationships.into_iter()), presenter);
            presenter.try_present( T::patch_one(&api::Entity::Id(id), patch, rels).into_future().wait())
        });
    }

    pub fn attach_post_one<T, Rel, P>(&mut self)
    where
        T: rel::raw::PostOne<P::Include, Rel>,
        Rel: rel::Relation,
        Rel::Resource: raw::RawUpdate + Deserialize,
        P: Presenter<Rel::Resource, R>
    {
        self.router.attach_post_one(T::resource_plural(), Rel::to_one(), |id, request, link_maker| {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(id.parse(), presenter);
            let post = try_status!(json::from_reader(request.attributes), presenter);
            let rels = try_status!(<<Rel::Resource as raw::RawUpdate>::Relationships as raw::UpdateRelationships>::from_iter(request.relationships.into_iter()), presenter);
            presenter.try_present(T::post_one(&api::Entity::Id(id), post, rels).into_future().wait())
        });
        self.router.attach_link_one(T::resource_plural(), Rel::to_one(), |id, rel, link_maker| {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(id.parse(), presenter);
            let rel_id = match rel {
                raw::Relationship::One(Some(identifier))  => {
                    try_status!(identifier.id.parse(), presenter)
                },
                _                                   => {
                    return presenter.present_err(Error::BadRequest)
                }
            };
            // TODO definitely wrong
            match T::link_one(&api::Entity::Id(id), &rel_id).into_future().wait() {
                Ok(_)       => presenter.present_err(Error::NoContent),
                Err(error)  => presenter.present_err(error),
            }
        })
    }

    pub fn attach_append_many<T, Rel, P>(&mut self)
    where
        T: rel::raw::Append<P::Include, Rel>,
        Rel: rel::Relation,
        Rel::Resource: raw::RawUpdate + Deserialize,
        P: Presenter<Rel::Resource, R>
    {
        self.router.attach_append_many(T::resource_plural(), Rel::to_many(), |id, request, link_maker| {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(id.parse(), presenter);
            let post = try_status!(json::from_reader(request.attributes), presenter);
            let rels = try_status!(<<Rel::Resource as raw::RawUpdate>::Relationships as raw::UpdateRelationships>::from_iter(request.relationships.into_iter()), presenter);
            presenter.try_present(T::append(&api::Entity::Id(id), post, rels).into_future().wait())
        });
        self.router.attach_append_link_many(T::resource_plural(), Rel::to_many(), |id, rel, link_maker| {
            let presenter = P::prepare(None, link_maker);
            let id = try_status!(id.parse(), presenter);
            let rel_ids: Vec<_> = match rel {
                raw::Relationship::Many(identifiers)   => {
                    let mut ids = vec![];
                    for identifier in identifiers {
                        ids.push(try_status!(identifier.id.parse(), presenter));
                    }
                    ids
                }
                _                                       => {
                    return presenter.present_err(Error::BadRequest)
                }
            };
            // TODO definitely wrong
            match T::append_links(&api::Entity::Id(id), &rel_ids).into_future().wait() {
                Ok(_)       => presenter.present_err(Error::NoContent),
                Err(error)  => presenter.present_err(error),
            }
        });
    }
}
