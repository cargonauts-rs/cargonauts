use api::Error;
use api::raw::{ResourceObject, Include, RawFetch, Relationship};
use links::LinkObject;
use presenter::Presenter;
use Serializer;
use Serialize;
use repr::SerializeRepr;
use router::{Response, Status};

pub struct JsonApi<R: Response> {
    response: R,
    field_set: Option<Vec<String>>,
}

impl<R: Response> JsonApi<R> {
    pub fn new(field_set: Option<Vec<String>>) -> JsonApi<R> {
        JsonApi {
            response: R::default(),
            field_set: field_set,
        }
    }

    fn respond(mut self, status: Status) -> R {
        self.response.set_status(status);
        self.response.set_content("application/vnd.api+json");
        self.response
    }

    fn split_up(&mut self) -> (&mut R::Serializer, Option<&[String]>) {
        let JsonApi { ref mut response, ref field_set } = *self;
        (response.serializer(), field_set.as_ref().map(|fields| &fields[..]))
    }

    fn serialize_resource<T>(&mut self, self_link: &str, resource: ResourceObject<T>, includes: Vec<Include<R::Serializer>>) -> Result<(), <R::Serializer as Serializer>::Error>
    where T: RawFetch {
        if includes.is_empty() {
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

    fn serialize_collection<T>(&mut self, self_link: &str, resources: Vec<ResourceObject<T>>, includes: Vec<Include<R::Serializer>>) -> Result<(), <R::Serializer as Serializer>::Error>
    where T: RawFetch {
        if includes.is_empty() {
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

    fn serialize_rel(&mut self, self_link: &str, rel_link: &str, rel: Relationship, includes: Vec<Include<R::Serializer>>) -> Result<(), <R::Serializer as Serializer>::Error> {
        fn serialize_rel<S: Serializer>(s: &mut S, state: &mut S::MapState, rel: Relationship) -> Result<(), S::Error> {
            match rel {
                Relationship::One(identifier)   => {
                    s.serialize_map_value(state, identifier)
                }
                Relationship::Many(identifiers) => {
                    s.serialize_map_value(state, identifiers)
                }
            }
        }
        if includes.is_empty() {
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

    fn serialize_compound_document<F>(&mut self, includes: Vec<Include<R::Serializer>>, links: LinkObject, data: F) -> Result<(), <R::Serializer as Serializer>::Error>
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
}

impl<R: Response> Presenter<R> for JsonApi<R> {
    fn present_resource<T>(mut self, self_link: &str, resource: ResourceObject<T>, includes: Vec<Include<R::Serializer>>) -> R
    where T: RawFetch {
        match self.serialize_resource(self_link, resource, includes) {
            Ok(())  => self.respond(Status::Ok),
            Err(_)  => self.respond(Status::BadRequest),
        }
    }

    fn present_collection<T>(mut self, self_link: &str, resources: Vec<ResourceObject<T>>, includes: Vec<Include<R::Serializer>>) -> R
    where T: RawFetch {
        match self.serialize_collection(self_link, resources, includes) {
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
        match self.serialize_rel(self_link, rel_link, rel, includes) {
            Ok(())  => self.respond(Status::Ok),
            Err(_)  => self.respond(Status::BadRequest),
        }
    }

    fn present_err(self, error: Error) -> R {
        unimplemented!()
    }
}

struct JsonApiObject;

impl Serialize for JsonApiObject {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let mut state = serializer.serialize_map(Some(1))?;
        serializer.serialize_map_key(&mut state, "version")?;
        serializer.serialize_map_value(&mut state, "1.0")?;
        serializer.serialize_map_end(state)
    }
}
