use api::Error;
use api::raw::{ResourceObject, Include, RawFetch, Relationship};
use links::LinkObject;
use presenter::Presenter;
use Serializer;
use repr::SerializeRepr;
use router::{Response, Status};

mod error;
mod include;
mod jsonapi;
mod linkage;
mod rels;
mod resource;

use self::error::ErrorObject;
use self::include::IncludesObject;
use self::jsonapi::JsonApiObject;
use self::linkage::{ToOneLinkage, ToManyLinkage};
use self::resource::{JsonApiResourceObject, JsonApiCollectionObject};

pub struct JsonApi<R: Response> {
    response: R,
    field_set: Option<Vec<String>>,
}

impl<R: Response> JsonApi<R> {
    fn respond(mut self, status: Status) -> R {
        self.response.set_status(status);
        self.response.set_content("application/vnd.api+json");
        self.response
    }

    fn split_up(&mut self) -> (&mut R::Serializer, Option<&[String]>) {
        let JsonApi { ref mut response, ref field_set } = *self;
        (response.serializer(), field_set.as_ref().map(|fields| &fields[..]))
    }

    fn serialize_resource<T>(&mut self, self_link: &str, resource: JsonApiResourceObject<T>, includes: IncludesObject<R::Serializer>) -> Result<(), <R::Serializer as Serializer>::Error>
    where T: RawFetch {
        if includes.0.is_empty() {
            self.serialize_document(LinkObject {
                self_link: Some(self_link),
                related_link: None,
            }, |serializer, mut state, field_set| serializer.serialize_map_value(state, SerializeRepr {
                repr: &resource,
                field_set: field_set,
            }))
        } else {
            self.serialize_compound_document(includes, LinkObject {
                self_link: Some(&self_link),
                related_link: None,
            }, |serializer, mut state, field_set| serializer.serialize_map_value(state, SerializeRepr {
                repr: &resource,
                field_set: field_set,
            }))
        }
    }

    fn serialize_collection<T>(&mut self, self_link: &str, resources: JsonApiCollectionObject<T>, includes: IncludesObject<R::Serializer>) -> Result<(), <R::Serializer as Serializer>::Error>
    where T: RawFetch {
        if includes.0.is_empty() {
            self.serialize_document(LinkObject {
                self_link: Some(self_link),
                related_link: None,
            }, |serializer, mut state, field_set| serializer.serialize_map_value(state, SerializeRepr {
                repr: &resources,
                field_set: field_set,
            }))
        } else {
            self.serialize_compound_document(includes, LinkObject {
                self_link: None,
                related_link: None,
            }, |serializer, mut state, field_set| serializer.serialize_map_value(state, SerializeRepr {
                repr: &resources,
                field_set: field_set,
            }))
        }
    }

    fn serialize_rel(&mut self, self_link: &str, rel_link: &str, rel: Relationship, includes: IncludesObject<R::Serializer>) -> Result<(), <R::Serializer as Serializer>::Error> {
        fn serialize_rel<S: Serializer>(s: &mut S, state: &mut S::MapState, rel: Relationship) -> Result<(), S::Error> {
            match rel {
                Relationship::One(identifier)   => {
                    s.serialize_map_value(state, ToOneLinkage(&identifier))
                }
                Relationship::Many(identifiers) => {
                    s.serialize_map_value(state, ToManyLinkage(&identifiers))
                }
            }
        }
        if includes.0.is_empty() {
            self.serialize_document(LinkObject {
                self_link: Some(self_link),
                related_link: Some(rel_link),
            }, |serializer, mut state, _| serialize_rel(serializer, &mut state, rel))
        } else {
            self.serialize_compound_document(includes, LinkObject {
                self_link: Some(self_link),
                related_link: Some(rel_link),
            }, |serializer, mut state, _| serialize_rel(serializer, &mut state, rel))
        }
    }

    fn serialize_nil(&mut self, self_link: &str) -> Result<(), <R::Serializer as Serializer>::Error> {
        self.serialize_document(LinkObject {
            self_link: Some(&self_link),
            related_link: None,
        }, |serializer, mut state, _| serializer.serialize_map_value(state, &()))
    }

    fn serialize_document<F>(&mut self, links: LinkObject, data: F) -> Result<(), <R::Serializer as Serializer>::Error>
    where F: FnOnce(&mut R::Serializer,
                    &mut <R::Serializer as Serializer>::MapState,
                    Option<&[String]>) -> Result<(), <R::Serializer as Serializer>::Error> {
        let (serializer, field_set) = self.split_up();
        let mut state = serializer.serialize_map(Some(3))?;
        serializer.serialize_map_key(&mut state, "links")?;
        serializer.serialize_map_value(&mut state, links)?;
        serializer.serialize_map_key(&mut state, "data")?;
        data(serializer, &mut state, field_set)?;
        serializer.serialize_map_key(&mut state, "jsonapi")?;
        serializer.serialize_map_value(&mut state, JsonApiObject)?;
        serializer.serialize_map_end(state)
    }

    fn serialize_compound_document<F>(&mut self, includes: IncludesObject<R::Serializer>, links: LinkObject, data: F) -> Result<(), <R::Serializer as Serializer>::Error>
    where F: FnOnce(&mut R::Serializer,
                    &mut <R::Serializer as Serializer>::MapState,
                    Option<&[String]>) -> Result<(), <R::Serializer as Serializer>::Error> {
        let (serializer, field_set) = self.split_up();
        let mut state = serializer.serialize_map(Some(4))?;
        serializer.serialize_map_key(&mut state, "links")?;
        serializer.serialize_map_value(&mut state, links)?;
        serializer.serialize_map_key(&mut state, "data")?;
        data(serializer, &mut state, field_set)?;
        serializer.serialize_map_key(&mut state, "includes")?;
        serializer.serialize_map_value(&mut state, SerializeRepr {
            repr: &includes,
            field_set: field_set,
        })?;
        serializer.serialize_map_key(&mut state, "jsonapi")?;
        serializer.serialize_map_value(&mut state, JsonApiObject)?;
        serializer.serialize_map_end(state)
    }

    fn serialize_err(&mut self, error: &Error) -> Result<(), <R::Serializer as Serializer>::Error> {
        let (serializer, _) = self.split_up();
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

impl<R: Response> Presenter<R> for JsonApi<R> {
    fn prepare(field_set: Option<Vec<String>>) -> Self {
        JsonApi {
            response: R::default(),
            field_set: field_set,
        }
    }

    fn present_resource<T>(mut self, self_link: &str, resource: ResourceObject<T>, includes: Vec<Include<R::Serializer>>) -> R
    where T: RawFetch {
        match self.serialize_resource(self_link, JsonApiResourceObject(&resource), IncludesObject(includes)) {
            Ok(())  => self.respond(Status::Ok),
            Err(_)  => self.respond(Status::BadRequest),
        }
    }

    fn present_collection<T>(mut self, self_link: &str, resources: Vec<ResourceObject<T>>, includes: Vec<Include<R::Serializer>>) -> R
    where T: RawFetch {
        match self.serialize_collection(self_link, JsonApiCollectionObject(resources), IncludesObject(includes)) {
            Ok(())  => self.respond(Status::Ok),
            Err(_)  => self.respond(Status::BadRequest),
        }
    }

    fn present_nil(mut self, self_link: &str) -> R {
        match self.serialize_nil(self_link) {
            Ok(())  => self.respond(Status::Ok),
            Err(_)  => self.respond(Status::BadRequest),
        }
    }

    fn present_rel(mut self, self_link: &str, rel_link: &str, rel: Relationship, includes: Vec<Include<R::Serializer>>) -> R {
        match self.serialize_rel(self_link, rel_link, rel, IncludesObject(includes)) {
            Ok(())  => self.respond(Status::Ok),
            Err(_)  => self.respond(Status::BadRequest),
        }
    }

    fn present_err(mut self, error: Error) -> R {
        match self.serialize_err(&error) {
            Ok(())  => self.respond(error.into()),
            Err(_)  => self.respond(Status::InternalError),
        }
    }
}
