use api::Error;
use api::raw::{ResourceResponse, CollectionResponse, RelResponse, ResourceObject, Include, RawFetch, Relationship};
use presenter::Presenter;
use Serializer;
use repr::{SerializeRepr, Represent, RepresentWith};
use router::{Response, Status, Linker};

mod error;
mod include;
mod jsonapi;
mod linkage;
mod links;
mod rels;
mod resource;

use self::error::ErrorObject;
use self::include::{JsonApiInclude, IncludesObject};
use self::jsonapi::JsonApiObject;
use self::linkage::{ToOneLinkage, ToManyLinkage};
use self::links::LinkObject;
use self::resource::{JsonApiResourceObject, JsonApiCollectionObject};

pub struct JsonApi<R: Response, L: Linker> {
    linker: L,
    response: R,
    field_set: Option<Vec<String>>,
}

struct JsonApiInner<'a, L: Linker + 'a> {
    linker: &'a L,
    field_set: Option<&'a [String]>,
}


impl<R: Response, L: Linker> JsonApi<R, L> {
    fn respond(mut self, status: Status) -> R {
        self.response.set_status(status);
        self.response.set_content("application/vnd.api+json");
        self.response
    }

    fn split_up(&mut self) -> (&mut R::Serializer, JsonApiInner<L>) {
        let JsonApi { ref mut response, ref field_set, ref linker } = *self;
        let serializer = response.serializer();
        let inner = JsonApiInner {
            linker: linker,
            field_set: field_set.as_ref().map(|f| &f[..]),
        };
        (serializer, inner)
    }
}

impl<'a, L: Linker> JsonApiInner<'a, L> {
    fn serialize_document<T, S>(self, serializer: &mut S, links: LinkObject, data: &T) -> Result<(), S::Error>
    where S: Serializer,
          T: RepresentWith<S> {
        let mut state = serializer.serialize_map(Some(3))?;
        serializer.serialize_map_key(&mut state, "links")?;
        serializer.serialize_map_value(&mut state, links)?;
        serializer.serialize_map_key(&mut state, "data")?;
        serializer.serialize_map_value(&mut state, SerializeRepr {
            repr: data,
            field_set: self.field_set,
        })?;
        serializer.serialize_map_key(&mut state, "jsonapi")?;
        serializer.serialize_map_value(&mut state, JsonApiObject)?;
        serializer.serialize_map_end(state)
    }

    fn serialize_compound_document<T, S>(self, serializer: &mut S, links: LinkObject, data: &T, includes: IncludesObject<S, L>) -> Result<(), S::Error>
    where S: Serializer,
          T: RepresentWith<S> {
        let mut state = serializer.serialize_map(Some(4))?;
        serializer.serialize_map_key(&mut state, "links")?;
        serializer.serialize_map_value(&mut state, links)?;
        serializer.serialize_map_key(&mut state, "data")?;
        serializer.serialize_map_value(&mut state, SerializeRepr {
            repr: data,
            field_set: self.field_set,
        })?;
        serializer.serialize_map_key(&mut state, "includes")?;
        serializer.serialize_map_value(&mut state, SerializeRepr {
            repr: &includes,
            field_set: self.field_set,
        })?;
        serializer.serialize_map_key(&mut state, "jsonapi")?;
        serializer.serialize_map_value(&mut state, JsonApiObject)?;
        serializer.serialize_map_end(state)
    }

    fn serialize_resource<T, S>(self, serializer: &mut S, resource: &ResourceObject<T>, includes: &[Include<JsonApiInclude<S>>]) -> Result<(), S::Error>
    where T: RawFetch + Represent, S: Serializer {
        let id = resource.id.to_string();
        let self_link = self.linker.resource(T::resource_plural(), &id);
        let links = LinkObject {
            self_link: Some(&self_link),
            related_link: None,
        };
        let resource = JsonApiResourceObject {
            resource: resource,
            linker: self.linker,
            id: &id,
            self_link: &self_link,
        };
        if includes.is_empty() {
            self.serialize_document(serializer, links, &resource)
        } else {
            let includes = IncludesObject {
                includes: includes,
                linker: self.linker,
            };
            self.serialize_compound_document(serializer, links, &resource, includes)
        }
    }

    fn serialize_collection<T, S>(self, serializer: &mut S, resources: &[ResourceObject<T>], includes: &[Include<JsonApiInclude<S>>]) -> Result<(), S::Error>
    where T: RawFetch + Represent, S: Serializer {
        let self_link = self.linker.collection(T::resource_plural());
        let links = LinkObject {
            self_link: Some(&self_link),
            related_link: None,
        };
        let collection = JsonApiCollectionObject {
            resources: resources,
            linker: self.linker,
        };
        if includes.is_empty() {
            self.serialize_document(serializer, links, &collection)
        } else {
            let includes = IncludesObject {
                includes: includes,
                linker: self.linker,
            };
            self.serialize_compound_document(serializer, links, &collection, includes)
        }
    }

    fn serialize_rel<S: Serializer>(self, serializer: &mut S, resource: &str, id: &str, name: &str, rel: &Relationship, includes: &[Include<JsonApiInclude<S>>]) -> Result<(),S::Error> {
        let self_link = self.linker.relationship(resource, id, name);
        let related_link = self.linker.related_resource(resource, id, name);
        let links = LinkObject {
            self_link: Some(&self_link),
            related_link: Some(&related_link),
        };
        match *rel {
            Relationship::One(ref identifier)   => {
                let linkage = ToOneLinkage(identifier);
                if includes.is_empty() {
                    self.serialize_document(serializer, links, &linkage)
                } else {
                    let includes = IncludesObject {
                        includes: includes,
                        linker: self.linker,
                    };
                    self.serialize_compound_document(serializer, links, &linkage, includes)
                }
            }
            Relationship::Many(ref identifiers) => {
                let linkage = ToManyLinkage(identifiers);
                if includes.is_empty() {
                    self.serialize_document(serializer, links, &linkage)
                } else {
                    let includes = IncludesObject {
                        includes: includes,
                        linker: self.linker,
                    };
                    self.serialize_compound_document(serializer, links, &linkage, includes)
                }
            }
        }
    }

    fn serialize_nil<S: Serializer>(self, serializer: &mut S, self_link: &str) -> Result<(), S::Error> {
        let links = LinkObject {
            self_link: Some(self_link),
            related_link: None,
        };
        self.serialize_document(serializer, links, &())
    }


    fn serialize_err<S: Serializer>(self, serializer: &mut S, error: &Error) -> Result<(), S::Error> {
        let mut state = serializer.serialize_map(Some(2))?;
        serializer.serialize_map_key(&mut state, "errors")?;
        serializer.serialize_map_value(&mut state, ErrorObject {
            status: error,
        })?;
        serializer.serialize_map_key(&mut state, "jsonapi")?;
        serializer.serialize_map_value(&mut state, JsonApiObject)?;
        serializer.serialize_map_end(state)
    }
}

impl<R: Response, L: Linker, T: RawFetch + Represent> Presenter<T> for JsonApi<R, L> {
    type Response = R;
    type Linker = L;
    type Include = JsonApiInclude<R::Serializer>;

    fn prepare(field_set: Option<Vec<String>>, linker: L) -> Self {
        JsonApi {
            response: R::default(),
            linker: linker,
            field_set: field_set,
        }
    }

    fn present_resource(mut self, response: ResourceResponse<Self::Include, T>) -> R {
        match {
            let (serializer, jsonapi) = self.split_up();
            jsonapi.serialize_resource(serializer, &response.resource, &response.includes)
        } {
            Ok(())  => self.respond(Status::Ok),
            Err(_)  => self.respond(Status::BadRequest),
        }
    }

    fn present_collection(mut self, response: CollectionResponse<Self::Include, T>) -> R {
        match {
            let (serializer, jsonapi) = self.split_up();
            jsonapi.serialize_collection(serializer, &response.resources, &response.includes) 
        } {
            Ok(())  => self.respond(Status::Ok),
            Err(_)  => self.respond(Status::BadRequest),
        }
    }

    fn present_nil(mut self, self_link: &str) -> R {
        match {
            let (serializer, jsonapi) = self.split_up();
            jsonapi.serialize_nil(serializer, self_link)
        } {
            Ok(())  => self.respond(Status::Ok),
            Err(_)  => self.respond(Status::BadRequest),
        }
    }

    fn present_rel(mut self, response: RelResponse<Self::Include>) -> R {
        match {
            let (serializer, jsonapi) = self.split_up();
            jsonapi.serialize_rel(serializer, response.resource, &response.id, response.related, &response.rel, &response.includes)
        } {
            Ok(())  => self.respond(Status::Ok),
            Err(_)  => self.respond(Status::BadRequest),
        }
    }

    fn present_err(mut self, error: Error) -> R {
        match {
            let (serializer, jsonapi) = self.split_up();
            jsonapi.serialize_err(serializer, &error)
        } {
            Ok(())  => self.respond(error.into()),
            Err(_)  => self.respond(Status::InternalError),
        }
    }
}
