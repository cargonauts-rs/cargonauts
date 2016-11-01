use api::{self, Error};
use api::rel;
use api::raw;
use router::Router as RouterTrait;
use Deserialize;
use presenter::{JsonApi, Presenter};

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

    pub fn attach_get<T: raw::RawGet>(&mut self) {
        self.router.attach_get(T::resource_plural(), |request| {
            let presenter = JsonApi::new(request.field_set);
            let id = try_status!(request.id.parse(), presenter);
            match T::get(id, &request.includes) {
                Ok(object)  => {
                    presenter.present_resource(&request.route, object.resource, object.includes)
                }
                Err(error)  => presenter.present_err(error),
            }
        });
    }

    pub fn attach_index<T: raw::RawIndex>(&mut self) {
        self.router.attach_index(T::resource_plural(), |request| {
            let presenter = JsonApi::new(request.field_set);
            match T::index(&request.includes, &request.sort, &request.page) {
                Ok(object)  => {
                    presenter.present_collection(&request.route, object.resources, object.includes)
                }
                Err(error)  => presenter.present_err(error),
            }
        });
    }

    pub fn attach_patch<T: raw::RawPatch>(&mut self) {
        self.router.attach_patch(T::resource_plural(), |request| {
            let presenter = JsonApi::new(request.field_set);
            let id = try_status!(request.id.parse(), presenter);
            let patch = try_status!(::from_value(request.attributes), presenter);
            let rels = try_status!(<<T as raw::RawUpdate>::Relationships as raw::UpdateRelationships>::from_iter(request.relationships.into_iter()), presenter);
            match T::patch(id, patch, rels) {
                Ok(object)  => {
                    presenter.present_resource(&request.route, object.resource, vec![])
                }
                Err(error)  => presenter.present_err(error),
            }
        });
    }

    pub fn attach_patch_async<T: raw::RawPatchAsync>(&mut self) {
        self.router.attach_patch(T::resource_plural(), |request| {
            let presenter = JsonApi::new(None);
            let id = try_status!(request.id.parse(), presenter);
            let patch = try_status!(::from_value(request.attributes), presenter);
            let rels = try_status!(<<T as raw::RawUpdate>::Relationships as raw::UpdateRelationships>::from_iter(request.relationships.into_iter()), presenter);
            match T::patch_async(id, patch, rels) {
                Ok(object)  => {
                    // TODO respond as accepted and set content location header
                    presenter.present_resource(&request.route, object.resource, vec![])
                }
                Err(error)  => presenter.present_err(error),
            }
        });
    }

    pub fn attach_post<T: raw::RawPost>(&mut self) {
        self.router.attach_post(T::resource_plural(), |request| {
            let presenter = JsonApi::new(request.field_set);
            let post = try_status!(::from_value(request.attributes), presenter);
            let rels = try_status!(<<T as raw::RawUpdate>::Relationships as raw::UpdateRelationships>::from_iter(request.relationships.into_iter()), presenter);
            match T::post(post, rels) {
                Ok(object)  => {
                    presenter.present_resource(&request.route, object.resource, vec![])
                }
                Err(error)  => presenter.present_err(error),
            }
        });
    }

    pub fn attach_post_async<T: raw::RawPostAsync>(&mut self) {
        self.router.attach_post(T::resource_plural(), |request| {
            let presenter = JsonApi::new(None);
            let post = try_status!(::from_value(request.attributes), presenter);
            let rels = try_status!(<<T as raw::RawUpdate>::Relationships as raw::UpdateRelationships>::from_iter(request.relationships.into_iter()), presenter);
            match T::post_async(post, rels) {
                Ok(object)  => {
                    // TODO respond as accepted and set content location header
                    presenter.present_resource(&request.route, object.resource, vec![])
                }
                Err(error)  => presenter.present_err(error),
            }
        });
    }

    pub fn attach_delete<T: api::Delete>(&mut self) {
        self.router.attach_delete(T::resource_plural(), |id| {
            let presenter = JsonApi::new(None);
            let id = try_status!(id.parse(), presenter);
            match T::delete(&id) {
                Ok(_)       => presenter.present_err(Error::NoContent),
                Err(error)  => presenter.present_err(error.into()),
            }
        });
    }

    pub fn attach_fetch_one<T, Rel>(&mut self)
    where T: rel::raw::FetchOne<Rel>, Rel: rel::Relation, Rel::Resource: raw::RawFetch {
        self.router.attach_fetch_one(T::resource_plural(), Rel::to_one(), |request| {
            let presenter = JsonApi::new(None);
            let id = try_status!(request.id.parse(), presenter);
            match T::fetch_one(&api::Entity::Id(id), &request.includes) {
                Ok(Some(object))    => {
                    presenter.present_resource(&request.route, object.resource, object.includes)
                }
                Ok(None)            => {
                    presenter.present_nil(&request.route)
                }
                Err(error)          => presenter.present_err(error),
            }
        });
        self.router.attach_fetch_rel(T::resource_plural(), Rel::to_one(), |request| {
            let presenter = JsonApi::new(None);
            let parsed_id = try_status!(request.id.parse(), presenter);
            match T::has_one(&api::Entity::Id(parsed_id)) {
                Ok(rel)         => {
                    let rel = raw::Relationship::One(rel.map(|id| raw::Identifier::new::<Rel::Resource>(&id)));
                    presenter.present_rel(&request.relationship_route, &request.related_resource_route, rel, vec![])
                }
                Err(error)          => presenter.present_err(error),
            }
        });
    }

    pub fn attach_fetch_many<T, Rel>(&mut self)
    where T: rel::raw::FetchMany<Rel>, Rel: rel::Relation, Rel::Resource: raw::RawFetch {
        self.router.attach_fetch_many(T::resource_plural(), Rel::to_many(), |request| {
            let presenter = JsonApi::new(None);
            let id = try_status!(request.id.parse(), presenter);
            match T::fetch_many(&api::Entity::Id(id), &request.includes) {
                Ok(object)     => {
                    presenter.present_collection(&request.route, object.resources, object.includes)
                }
                Err(error)          => presenter.present_err(error),
            }
        });
        self.router.attach_fetch_rel(T::resource_plural(), Rel::to_many(), |request| {
            let presenter = JsonApi::new(None);
            let parsed_id = try_status!(request.id.parse(), presenter);
            match T::has_many(&api::Entity::Id(parsed_id)) {
                Ok(rel)         => {
                    let rel = raw::Relationship::Many(rel.into_iter().map(|id| raw::Identifier::new::<Rel::Resource>(&id)).collect());
                    presenter.present_rel(&request.relationship_route, &request.related_resource_route, rel, vec![])
                }
                Err(error)          => presenter.present_err(error),
            }
        });
    }

    pub fn attach_delete_one<T: rel::raw::DeleteOne<Rel>, Rel: rel::Relation>(&mut self) {
        self.router.attach_delete_one(T::resource_plural(), Rel::to_one(), |id| {
            let presenter = JsonApi::new(None);
            let id = try_status!(id.parse(), presenter);
            match T::delete_one(&api::Entity::Id(id)) {
                Ok(_)       => presenter.present_err(Error::NoContent),
                Err(error)  => presenter.present_err(error),
            }
        });
        self.router.attach_delete_one_rel(T::resource_plural(), Rel::to_one(), |id| {
            let presenter = JsonApi::new(None);
            let parsed_id = try_status!(id.parse(), presenter);
            match T::unlink_one(&api::Entity::Id(parsed_id)) {
                Ok(_)       => presenter.present_err(Error::NoContent),
                Err(error)  => presenter.present_err(error),
            }
        });
    }

    pub fn attach_clear_many<T: rel::raw::Clear<Rel>, Rel: rel::Relation>(&mut self) {
        self.router.attach_clear_many(T::resource_plural(), Rel::to_many(), |id| {
            let presenter = JsonApi::new(None);
            let id = try_status!(id.parse(), presenter);
            match T::clear_many(&api::Entity::Id(id)) {
                Ok(_)       => presenter.present_err(Error::NoContent),
                Err(error)  => presenter.present_err(error),
            }
        });
        self.router.attach_clear_many_rel(T::resource_plural(), Rel::to_many(), |id| {
            let presenter = JsonApi::new(None);
            let id = try_status!(id.parse(), presenter);
            match T::unlink_all(&api::Entity::Id(id)) {
                Ok(_)       => presenter.present_err(Error::NoContent),
                Err(error)  => presenter.present_err(error),
            }
        });
    }

    pub fn attach_remove_many<T: rel::raw::Remove<Rel>, Rel: rel::Relation>(&mut self) {
        self.router.attach_remove_many(T::resource_plural(), Rel::to_many(), |id, rel_ids| {
            let presenter = JsonApi::new(None);
            let id = try_status!(id.parse(), presenter);
            let parsed_rel_ids = try_status!(rel_ids.iter().map(|id| id.parse()).collect::<Result<Vec<_>, _>>(), presenter);
            match T::remove_many(&api::Entity::Id(id), &parsed_rel_ids) {
                Ok(_)       => presenter.present_err(Error::NoContent),
                Err(error)  => presenter.present_err(error),
            }
        });
        self.router.attach_remove_many_rel(T::resource_plural(), Rel::to_many(), |id, rel_ids| {
            let presenter = JsonApi::new(None);
            let parsed_id = try_status!(id.parse(), presenter);
            let parsed_rel_ids = try_status!(rel_ids.iter().map(|id| id.parse()).collect::<Result<Vec<_>, _>>(), presenter);
            match T::unlink_many(&api::Entity::Id(parsed_id), &parsed_rel_ids) {
                Ok(_)       => presenter.present_err(Error::NoContent),
                Err(error)  => presenter.present_err(error),
            }
        });
    }

    pub fn attach_patch_one<T, Rel>(&mut self)
    where T: rel::raw::PatchOne<Rel>, Rel: rel::Relation, Rel::Resource: raw::RawUpdate {
        self.router.attach_patch_one(T::resource_plural(), Rel::to_one(), |request| {
            let presenter = JsonApi::new(None);
            let id = try_status!(request.id.parse(), presenter);
            let patch = try_status!(::from_value(request.attributes), presenter);
            let rels = try_status!(<<Rel::Resource as raw::RawUpdate>::Relationships as raw::UpdateRelationships>::from_iter(request.relationships.into_iter()), presenter);
            match T::patch_one(&api::Entity::Id(id), patch, rels) {
                Ok(Some(object))    => {
                    presenter.present_resource(&request.route, object.resource, vec![])
                }
                Ok(None)            => {
                    presenter.present_nil(&request.route)
                }
                Err(error)          => presenter.present_err(error),
            }
        });
    }

    pub fn attach_post_one<T, Rel>(&mut self)
    where T: rel::raw::PostOne<Rel>, Rel: rel::Relation, Rel::Resource: raw::RawUpdate + Deserialize {
        self.router.attach_post_one(T::resource_plural(), Rel::to_one(), |id, request| {
            let presenter = JsonApi::new(None);
            let id = try_status!(id.parse(), presenter);
            let post = try_status!(::from_value(request.attributes), presenter);
            let rels = try_status!(<<Rel::Resource as raw::RawUpdate>::Relationships as raw::UpdateRelationships>::from_iter(request.relationships.into_iter()), presenter);
            match T::post_one(&api::Entity::Id(id), post, rels) {
                Ok(object)  => {
                    presenter.present_resource(&request.route, object.resource, vec![])
                }
                Err(error)  => presenter.present_err(error),
            }
        });
        self.router.attach_link_one(T::resource_plural(), Rel::to_one(), |id, rel| {
            let presenter = JsonApi::new(None);
            let id = try_status!(id.parse(), presenter);
            let rel_id = match rel {
                raw::Relationship::One(Some(identifier))  => {
                    try_status!(identifier.id.parse(), presenter)
                },
                _                                   => {
                    return presenter.present_err(Error::BadRequest)
                }
            };
            match T::link_one(&api::Entity::Id(id), &rel_id) {
                Ok(_)       => presenter.present_err(Error::NoContent),
                Err(error)  => presenter.present_err(error),
            }
        })
    }

    pub fn attach_append_many<T, Rel>(&mut self)
    where T: rel::raw::Append<Rel>, Rel: rel::Relation, Rel::Resource: raw::RawUpdate + Deserialize {
        self.router.attach_append_many(T::resource_plural(), Rel::to_many(), |id, request| {
            let presenter = JsonApi::new(None);
            let id = try_status!(id.parse(), presenter);
            let post = try_status!(::from_value(request.attributes), presenter);
            let rels = try_status!(<<Rel::Resource as raw::RawUpdate>::Relationships as raw::UpdateRelationships>::from_iter(request.relationships.into_iter()), presenter);
            match T::append(&api::Entity::Id(id), post, rels) {
                Ok(object)  => {
                    presenter.present_resource(&request.route, object.resource, vec![])
                }
                Err(error)  => presenter.present_err(error),
            }
        });
        self.router.attach_append_link_many(T::resource_plural(), Rel::to_many(), |id, rel| {
            let presenter = JsonApi::new(None);
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
            match T::append_links(&api::Entity::Id(id), &rel_ids) {
                Ok(_)       => presenter.present_err(Error::NoContent),
                Err(error)  => presenter.present_err(error),
            }
        });
    }
}
