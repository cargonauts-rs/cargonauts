use api;
use api::rel;
use api::raw;
use BASE_URL;
use links::make_link;
use router::Router as RouterTrait;
use router::{Status, Response};
use Deserialize;
use _internal::document::*;
use repr::RepresentWith;


macro_rules! try_status {
    ($x:expr, $r:expr)  => {
        match $x {
            Ok(x)   => x,
            Err(_)  => {
                $r.set_status(Status::Conflict);
                return $r
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
            let mut response = R::Response::default();
            let id = try_status!(request.id.parse(), response);
            match T::get(id, &request.includes) {
                Ok(object)      => {
                    let document = ResourceDocument::new(object.resource, object.includes);
                    respond_with(document, &mut response);
                }
                Err(ref error)  => response.set_status(error.into()),
            }
            response
        });
    }

    pub fn attach_index<T: raw::RawIndex>(&mut self) {
        self.router.attach_index(T::resource_plural(), |request| {
            let mut response = R::Response::default();
            match T::index(&request.includes, &request.sort, &request.page) {
                Ok(object)      => {
                    let reprs = object.resources.into_iter().collect();
                    let document = CollectionDocument::new(reprs, object.includes);
                    respond_with(document, &mut response);
                }
                Err(ref error)  => response.set_status(error.into()),
            }
            response
        });
    }

    pub fn attach_patch<T: raw::RawPatch>(&mut self) {
        self.router.attach_patch(T::resource_plural(), |request| {
            let mut response = R::Response::default();
            let id = try_status!(request.id.parse(), response);
            let patch = try_status!(::from_value(request.attributes), response);
            let rels = try_status!(<<T as raw::RawUpdate>::Relationships as raw::UpdateRelationships>::from_iter(request.relationships.into_iter()), response);
            match T::patch(id, patch, rels) {
                Ok(object)      => {
                    let document = ResourceDocument::new(object.resource, vec![]);
                    respond_with(document, &mut response);
                }
                Err(ref error)  => response.set_status(error.into()),
            }
            response
        });
    }

    pub fn attach_patch_async<T: raw::RawPatchAsync>(&mut self) {
        self.router.attach_patch(T::resource_plural(), |request| {
            let mut response = R::Response::default();
            let id = try_status!(request.id.parse(), response);
            let patch = try_status!(::from_value(request.attributes), response);
            let rels = try_status!(<<T as raw::RawUpdate>::Relationships as raw::UpdateRelationships>::from_iter(request.relationships.into_iter()), response);
            match T::patch_async(id, patch, rels) {
                Ok(object)      => {
                    let document = ResourceDocument::new(object.resource, vec![]);
                    // TODO respond as accepted and set content location header
                    respond_with(document, &mut response);
                }
                Err(ref error)  => response.set_status(error.into()),
            }
            response
        });
    }

    pub fn attach_post<T: raw::RawPost>(&mut self) {
        self.router.attach_post(T::resource_plural(), |request| {
            let mut response = R::Response::default();
            let post = try_status!(::from_value(request.attributes), response);
            let rels = try_status!(<<T as raw::RawUpdate>::Relationships as raw::UpdateRelationships>::from_iter(request.relationships.into_iter()), response);
            match T::post(post, rels) {
                Ok(object)      => {
                    let document = ResourceDocument::new(object.resource, vec![]);
                    respond_with(document, &mut response);
                }
                Err(ref error)  => response.set_status(error.into()),
            }
            response
        });
    }

    pub fn attach_post_async<T: raw::RawPostAsync>(&mut self) {
        self.router.attach_post(T::resource_plural(), |request| {
            let mut response = R::Response::default();
            let post = try_status!(::from_value(request.attributes), response);
            let rels = try_status!(<<T as raw::RawUpdate>::Relationships as raw::UpdateRelationships>::from_iter(request.relationships.into_iter()), response);
            match T::post_async(post, rels) {
                Ok(object)      => {
                    let document = ResourceDocument::new(object.resource, vec![]);
                    // TODO respond as accepted and set content location header
                    respond_with(document, &mut response);
                }
                Err(ref error)  => response.set_status(error.into()),
            }
            response
        });
    }

    pub fn attach_delete<T: api::Delete>(&mut self) {
        self.router.attach_delete(T::resource_plural(), |id| {
            let mut response = R::Response::default();
            let id = try_status!(id.parse(), response);
            match T::delete(&id) {
                Ok(_)           => response.set_status(Status::NoContent),
                Err(ref error)  => response.set_status(error.into()),
            }
            response
        });
    }

    pub fn attach_fetch_one<T, Rel>(&mut self)
    where T: rel::raw::FetchOne<Rel>, Rel: rel::Relation, Rel::Resource: raw::RawFetch {
        self.router.attach_fetch_one(T::resource_plural(), Rel::to_one(), |request| {
            let mut response = R::Response::default();
            let id = try_status!(request.id.parse(), response);
            match T::fetch_one(&api::Entity::Id(id), &request.includes) {
                Ok(Some(object))    => {
                    let document = ResourceDocument::new(object.resource, object.includes);
                    respond_with(document, &mut response);
                }
                Ok(None)            => {
                    let document = NullDocument::new(make_link(&[
                        BASE_URL,
                        T::resource_plural(),
                        &request.id,
                        Rel::to_one(),
                    ]));
                    respond_with(document, &mut response);
                }
                Err(ref error)      => response.set_status(error.into()),
            }
            response
        });
        self.router.attach_fetch_rel(T::resource_plural(), Rel::to_one(), |id| {
            let mut response = R::Response::default();
            let parsed_id = try_status!(id.parse(), response);
            match T::has_one(&api::Entity::Id(parsed_id)) {
                Ok(rel)         => {
                    let rel = raw::Relationship::One(rel.map(|id| raw::Identifier::new::<Rel::Resource>(&id)));
                    let document = RelDocument::new(rel, T::resource_plural(), id, Rel::to_one());
                    respond_with(document, &mut response);
                }
                Err(ref error)  => response.set_status(error.into()),
            }
            response
        });
    }

    pub fn attach_fetch_many<T, Rel>(&mut self)
    where T: rel::raw::FetchMany<Rel>, Rel: rel::Relation, Rel::Resource: raw::RawFetch {
        self.router.attach_fetch_many(T::resource_plural(), Rel::to_many(), |request| {
            let mut response = R::Response::default();
            let id = try_status!(request.id.parse(), response);
            match T::fetch_many(&api::Entity::Id(id), &request.includes) {
                Ok(object)     => {
                    let reprs = object.resources.into_iter().collect();
                    let document = CollectionDocument::new(reprs, object.includes);
                    respond_with(document, &mut response);
                }
                Err(ref error)      => response.set_status(error.into()),
            }
            response
        });
        self.router.attach_fetch_rel(T::resource_plural(), Rel::to_many(), |id| {
            let mut response = R::Response::default();
            let parsed_id = try_status!(id.parse(), response);
            match T::has_many(&api::Entity::Id(parsed_id)) {
                Ok(rel)         => {
                    let rel = raw::Relationship::Many(rel.into_iter().map(|id| raw::Identifier::new::<Rel::Resource>(&id)).collect());
                    let document = RelDocument::new(rel, T::resource_plural(), id, Rel::to_many());
                    respond_with(document, &mut response);
                }
                Err(ref error)  => response.set_status(error.into()),
            }
            response
        });
    }

    pub fn attach_delete_one<T: rel::raw::DeleteOne<Rel>, Rel: rel::Relation>(&mut self) {
        self.router.attach_delete_one(T::resource_plural(), Rel::to_one(), |id| {
            let mut response = R::Response::default();
            let id = try_status!(id.parse(), response);
            match T::delete_one(&api::Entity::Id(id)) {
                Ok(_)           => response.set_status(Status::NoContent),
                Err(ref error)  => response.set_status(error.into()),
            }
            response
        });
        self.router.attach_delete_one_rel(T::resource_plural(), Rel::to_one(), |id| {
            let mut response = R::Response::default();
            let parsed_id = try_status!(id.parse(), response);
            match T::unlink_one(&api::Entity::Id(parsed_id)) {
                Ok(_)           => response.set_status(Status::NoContent),
                Err(ref error)  => response.set_status(error.into()),
            }
            response
        });
    }

    pub fn attach_clear_many<T: rel::raw::Clear<Rel>, Rel: rel::Relation>(&mut self) {
        self.router.attach_clear_many(T::resource_plural(), Rel::to_many(), |id| {
            let mut response = R::Response::default();
            let id = try_status!(id.parse(), response);
            match T::clear_many(&api::Entity::Id(id)) {
                Ok(_)           => response.set_status(Status::NoContent),
                Err(ref error)  => response.set_status(error.into()),
            }
            response
        });
        self.router.attach_clear_many_rel(T::resource_plural(), Rel::to_many(), |id| {
            let mut response = R::Response::default();
            let id = try_status!(id.parse(), response);
            match T::unlink_all(&api::Entity::Id(id)) {
                Ok(_)           => response.set_status(Status::NoContent),
                Err(ref error)  => response.set_status(error.into()),
            }
            response
        });
    }

    pub fn attach_remove_many<T: rel::raw::Remove<Rel>, Rel: rel::Relation>(&mut self) {
        self.router.attach_remove_many(T::resource_plural(), Rel::to_many(), |id, rel_ids| {
            let mut response = R::Response::default();
            let id = try_status!(id.parse(), response);
            let parsed_rel_ids = try_status!(rel_ids.iter().map(|id| id.parse()).collect::<Result<Vec<_>, _>>(), response);
            match T::remove_many(&api::Entity::Id(id), &parsed_rel_ids) {
                Ok(_)           => response.set_status(Status::NoContent),
                Err(ref error)  => response.set_status(error.into()),
            }
            response
        });
        self.router.attach_remove_many_rel(T::resource_plural(), Rel::to_many(), |id, rel_ids| {
            let mut response = R::Response::default();
            let parsed_id = try_status!(id.parse(), response);
            let parsed_rel_ids = try_status!(rel_ids.iter().map(|id| id.parse()).collect::<Result<Vec<_>, _>>(), response);
            match T::unlink_many(&api::Entity::Id(parsed_id), &parsed_rel_ids) {
                Ok(_)           => response.set_status(Status::NoContent),
                Err(ref error)  => response.set_status(error.into()),
            }
            response
        });
    }

    pub fn attach_patch_one<T, Rel>(&mut self)
    where T: rel::raw::PatchOne<Rel>, Rel: rel::Relation, Rel::Resource: raw::RawUpdate {
        self.router.attach_patch_one(T::resource_plural(), Rel::to_one(), |request| {
            let mut response = R::Response::default();
            let id = try_status!(request.id.parse(), response);
            let patch = try_status!(::from_value(request.attributes), response);
            let rels = try_status!(<<Rel::Resource as raw::RawUpdate>::Relationships as raw::UpdateRelationships>::from_iter(request.relationships.into_iter()), response);
            match T::patch_one(&api::Entity::Id(id), patch, rels) {
                Ok(Some(patch)) => {
                    let document = ResourceDocument::new(patch.resource, vec![]);
                    respond_with(document, &mut response);
                }
                Ok(None)        => response.set_status(Status::NoContent),
                Err(ref error)  => response.set_status(error.into()),
            }
            response
        });
    }

    pub fn attach_post_one<T, Rel>(&mut self)
    where T: rel::raw::PostOne<Rel>, Rel: rel::Relation, Rel::Resource: raw::RawUpdate + Deserialize {
        self.router.attach_post_one(T::resource_plural(), Rel::to_one(), |id, request| {
            let mut response = R::Response::default();
            let id = try_status!(id.parse(), response);
            let post = try_status!(::from_value(request.attributes), response);
            let rels = try_status!(<<Rel::Resource as raw::RawUpdate>::Relationships as raw::UpdateRelationships>::from_iter(request.relationships.into_iter()), response);
            match T::post_one(&api::Entity::Id(id), post, rels) {
                Ok(post)    => {
                    let document = ResourceDocument::new(post.resource, vec![]);
                    respond_with(document, &mut response);
                }
                Err(ref error)  => response.set_status(error.into()),
            }
            response
        });
        self.router.attach_link_one(T::resource_plural(), Rel::to_one(), |id, rel| {
            let mut response = R::Response::default();
            let id = try_status!(id.parse(), response);
            let rel_id = match rel {
                raw::Relationship::One(Some(identifier))  => {
                    try_status!(identifier.id.parse(), response)
                },
                _                                   => {
                    response.set_status(Status::BadRequest);
                    return response
                }
            };
            match T::link_one(&api::Entity::Id(id), &rel_id) {
                Ok(_)           => response.set_status(Status::NoContent),
                Err(ref error)  => response.set_status(error.into()),
            }
            response
        })
    }

    pub fn attach_append_many<T, Rel>(&mut self)
    where T: rel::raw::Append<Rel>, Rel: rel::Relation, Rel::Resource: raw::RawUpdate + Deserialize {
        self.router.attach_append_many(T::resource_plural(), Rel::to_many(), |id, request| {
            let mut response = R::Response::default();
            let id = try_status!(id.parse(), response);
            let post = try_status!(::from_value(request.attributes), response);
            let rels = try_status!(<<Rel::Resource as raw::RawUpdate>::Relationships as raw::UpdateRelationships>::from_iter(request.relationships.into_iter()), response);
            match T::append(&api::Entity::Id(id), post, rels) {
                Ok(post)    => {
                    let document = ResourceDocument::new(post.resource, vec![]);
                    respond_with(document, &mut response);
                }
                Err(ref error)  => response.set_status(error.into()),
            }
            response
        });
        self.router.attach_append_link_many(T::resource_plural(), Rel::to_many(), |id, rel| {
            let mut response = R::Response::default();
            let id = try_status!(id.parse(), response);
            let rel_ids = match rel {
                raw::Relationship::Many(identifiers)   => {
                    let mut ids = vec![];
                    for identifier in identifiers {
                        ids.push(try_status!(identifier.id.parse(), response));
                    }
                    ids
                }
                _                                       => {
                    response.set_status(Status::BadRequest);
                    return response
                }
            };
            match T::append_links(&api::Entity::Id(id), &rel_ids) {
                Ok(_)           => response.set_status(Status::NoContent),
                Err(ref error)  => response.set_status(error.into()),
            }
            response
        });
    }
}

fn respond_with<T, R>(document: T, response: &mut R)
    where R: Response,
          T: RepresentWith<R::Serializer>,
{
    match document.repr_with(response.serializer()) {
        Ok(_)   => response.set_status(Status::Ok),
        // TODO write the error to the body in the error case
        Err(_)  => response.set_status(Status::InternalError),
    };
}
