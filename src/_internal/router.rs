use api;
use from_value;
use Serialize;
use router::Router as RouterTrait;
use router::{PostObject, PatchObject, Status, Response};
use _internal::{Resource, Wrapper};
use _internal::document::*;

macro_rules! parse_id {
    ($id:expr, $response:expr)  => {
        match $id.parse() {
            Ok(id)  => id,
            Err(_)  => {
                $response.set_status(Status::Conflict);
                // TODO write the error to the body in the error case
                return $response;
            }
        };
    }
}

pub struct Router<'a, R: RouterTrait + 'a> {
    router: &'a mut R,
    base_url: &'static str,
}

impl<'a, R: RouterTrait> Router<'a, R> {
    pub fn new(router: &'a mut R) -> Router<'a, R> {
        let base_url = router.base_url();
        Router {
            router: router,
            base_url: base_url,
        }
    }

    pub fn attach_delete<T: api::Delete>(&mut self) where Resource<T>: Wrapper<T> {
        self.router.attach_delete(T::resource(), |id| {
            let mut response = R::Response::default();
            let id = parse_id!(id, response);
            match T::delete(id) {
                Ok(())                                  => response.set_status(Status::NoContent),
                Err(api::DeleteError::BadRequest)       => response.set_status(Status::BadRequest),
                Err(api::DeleteError::Forbidden)        => response.set_status(Status::Forbidden),
                Err(api::DeleteError::NotFound)         => response.set_status(Status::NotFound),
                Err(api::DeleteError::InternalError)    => response.set_status(Status::InternalError),
            }
            response
        });
    }

    pub fn attach_get<T: api::Get>(&mut self) where Resource<T>: Wrapper<T> {
        let Router { ref mut router, base_url } = *self;
        router.attach_get(T::resource(), |get_object| {
            let mut response = R::Response::default();
            let id = parse_id!(get_object.id, response);
            if let Some(resource) = T::get(id) {
                let document = ResourceDocument::new(resource, &get_object.includes, base_url);
                respond_with(document, response)
            } else {
                // TODO write the error to the body in the error case
                response.set_status(Status::NotFound);
                response
            }
        });
    }

    pub fn attach_index<T: api::Index>(&mut self) where Resource<T>: Wrapper<T> {
        let Router { ref mut router, base_url } = *self;
        router.attach_index(T::resource(), |index_object| {
            let mut response = R::Response::default();
            let resources = T::index();
            let document = CollectionDocument::new(resources, &index_object.includes, base_url);
            match document.serialize(response.serializer()) {
                Ok(_)   => response.set_status(Status::Ok),
                // TODO write the error to the body in the error case
                Err(_)  => response.set_status(Status::InternalError),
            }
            response
        });
    }

    pub fn attach_patch<T: api::Patch>(&mut self) where Resource<T>: Wrapper<T> {
        let Router { ref mut router, base_url } = *self;
        router.attach_patch(T::resource(), |patch_object| {
            let mut response = R::Response::default();
            if patch_object.resource_type != T::resource() {
                response.set_status(Status::Conflict);
                // TODO write the error to the body in the error case
                return response
            }
            let id = parse_id!(patch_object.id, response);
            let PatchObject { attributes, relationships, .. } = patch_object;
            let attributes = match from_value::<T::Patch>(attributes) {
                Ok(attributes)  => attributes,
                Err(_)          => {
                    response.set_status(Status::Conflict);
                    // TODO write the error to the body in the error case
                    return response
                }
            };
            match T::patch(id, attributes) {
                Ok(Some(resource))  => {
                    if let Err(err) = <Resource<T> as Wrapper<T>>::put_related(&resource.id(), &relationships) {
                        match err {
                            api::LinkError::BadRequest     => response.set_status(Status::BadRequest),
                            api::LinkError::Forbidden      => response.set_status(Status::Forbidden),
                            api::LinkError::NotFound       => response.set_status(Status::NotFound),
                            api::LinkError::Conflict       => response.set_status(Status::Conflict),
                            api::LinkError::InternalError  => response.set_status(Status::InternalError),
                        }
                        return response
                    }
                    let document = ResourceDocument::new(resource, &[], base_url);
                    respond_with(document, response)
                }
                Ok(None)            => {
                    response.set_status(Status::NoContent);
                    response
                }
                Err(err)            => {
                    // TODO write the error to the body in the error case
                    match err {
                        api::PatchError::BadRequest     => response.set_status(Status::BadRequest),
                        api::PatchError::Forbidden      => response.set_status(Status::Forbidden),
                        api::PatchError::NotFound       => response.set_status(Status::NotFound),
                        api::PatchError::Conflict       => response.set_status(Status::Conflict),
                        api::PatchError::InternalError  => response.set_status(Status::InternalError),
                    }
                    response
                }
            }
        });
    }

    pub fn attach_post<T: api::Post>(&mut self) where Resource<T>: Wrapper<T> {
        let Router { ref mut router, base_url } = *self;
        router.attach_post(T::resource(), |post_object| {
            let mut response = R::Response::default();
            if post_object.resource_type != T::resource() {
                response.set_status(Status::Conflict);
                // TODO write the error to the body in the error case
                return response
            }
            let PostObject { attributes, relationships, .. } = post_object;
            let attributes = match from_value::<T>(attributes) {
                Ok(attributes)  => attributes,
                Err(_)          => {
                    response.set_status(Status::Conflict);
                    // TODO write the error to the body in the error case
                    return response
                }
            };
            match attributes.post() {
                Ok(Some(resource))  => {
                    if let Err(err) = <Resource<T> as Wrapper<T>>::put_related(&resource.id(), &relationships) {
                        match err {
                            api::LinkError::BadRequest     => response.set_status(Status::BadRequest),
                            api::LinkError::Forbidden      => response.set_status(Status::Forbidden),
                            api::LinkError::NotFound       => response.set_status(Status::NotFound),
                            api::LinkError::Conflict       => response.set_status(Status::Conflict),
                            api::LinkError::InternalError  => response.set_status(Status::InternalError),
                        }
                        return response
                    }
                    let document = ResourceDocument::new(resource, &[], base_url);
                    respond_with(document, response)
                }
                Ok(None)            => {
                    response.set_status(Status::NoContent);
                    response
                }
                Err(err)            => {
                    // TODO write the error to the body in the error case
                    match err {
                        api::PostError::BadRequest      => response.set_status(Status::BadRequest),
                        api::PostError::Forbidden       => response.set_status(Status::Forbidden),
                        api::PostError::Conflict        => response.set_status(Status::Conflict),
                        api::PostError::InternalError   => response.set_status(Status::InternalError),
                    }
                    response
                }
            }
        });
    }

    pub fn attach_get_has_one<T: api::HasOne<Rel>, Rel: api::Resource>(&mut self) where Resource<Rel>: Wrapper<Rel> {
        let Router { ref mut router, base_url } = *self;
        router.attach_get_rel(T::resource(), Rel::resource(), |id| {
            let mut response = R::Response::default();
            let parsed_id = parse_id!(id, response);
            match <T as api::HasOne<Rel>>::has_one(&parsed_id) {
                Some(related)   => {
                    let document = HasOneDocument::new(related, base_url, T::resource(), &id);
                    respond_with(document, response)
                }
                None            => {
                    response.set_status(Status::NotFound);
                    response
                }
            }
        });
    }

    pub fn attach_get_has_many<T: api::HasMany<Rel>, Rel: api::Resource>(&mut self) where Resource<Rel>: Wrapper<Rel> {
        let Router { ref mut router, base_url } = *self;
        router.attach_get_rel(T::resource(), Rel::resource(), |id| {
            let mut response = R::Response::default();
            let parsed_id = parse_id!(id, response);
            let related = <T as api::HasMany<Rel>>::has_many(&parsed_id);
            let document = HasManyDocument::<Rel>::new(related, base_url, T::resource(), &id);
            respond_with(document, response)
        })
    }
}

fn respond_with<T: Serialize, R: Response>(document: T, mut response: R) -> R {
    match document.serialize(response.serializer()) {
        Ok(_)   => response.set_status(Status::Ok),
        // TODO write the error to the body in the error case
        Err(_)  => response.set_status(Status::InternalError),
    };
    response
}
