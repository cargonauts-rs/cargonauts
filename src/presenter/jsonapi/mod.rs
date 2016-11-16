use io_adapter::WriteAdapter;
use std::marker::PhantomData;

use api::Error;
use api::raw::{ResourceResponse, CollectionResponse, RelResponse, RawFetch, Relationship};
use presenter::Presenter;
use Serializer;
use repr::{SerializeRepr, Represent, RepresentWith};
use router::{Router, Response, Status, MakeLinks};

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

pub struct JsonApi<R: Router, S: Serializer + WriteAdapter<R::Response>> {
    link_maker: R::LinkMaker,
    field_set: Option<Vec<String>>,
    _spoopy: PhantomData<S>,
}

impl<R, S> JsonApi<R, S>
where
    R: Router,
    S: Serializer + WriteAdapter<R::Response>,
{
    fn respond<F>(self, status: Status, f: F) -> R::Response
    where
        F: FnOnce(Self, &mut S) -> Result<(), S::Error>
    {
        let mut response = R::Response::default();
        response.set_status(status);
        response.set_content("application/vnd.api+json");
        let mut serializer = S::wrap(response);
        f(self, &mut serializer).unwrap();
        serializer.into_inner()
    }

    fn serialize_document<T>(&self, serializer: &mut S, links: LinkObject, data: &T) -> Result<(), S::Error>
    where T: RepresentWith<S> {
        let mut state = serializer.serialize_map(Some(3))?;
        serializer.serialize_map_key(&mut state, "links")?;
        serializer.serialize_map_value(&mut state, links)?;
        serializer.serialize_map_key(&mut state, "data")?;
        serializer.serialize_map_value(&mut state, SerializeRepr {
            repr: data,
            field_set: self.field_set.as_ref().map(|s| &s[..]),
        })?;
        serializer.serialize_map_key(&mut state, "jsonapi")?;
        serializer.serialize_map_value(&mut state, JsonApiObject)?;
        serializer.serialize_map_end(state)
    }

    fn serialize_compound_document<T>(&self, serializer: &mut S, links: LinkObject, data: &T, includes: IncludesObject<S, R::LinkMaker>) -> Result<(), S::Error>
    where T: RepresentWith<S> {
        let mut state = serializer.serialize_map(Some(4))?;
        serializer.serialize_map_key(&mut state, "links")?;
        serializer.serialize_map_value(&mut state, links)?;
        serializer.serialize_map_key(&mut state, "data")?;
        serializer.serialize_map_value(&mut state, SerializeRepr {
            repr: data,
            field_set: self.field_set.as_ref().map(|s| &s[..]),
        })?;
        serializer.serialize_map_key(&mut state, "includes")?;
        serializer.serialize_map_value(&mut state, SerializeRepr {
            repr: &includes,
            field_set: self.field_set.as_ref().map(|s| &s[..]),
        })?;
        serializer.serialize_map_key(&mut state, "jsonapi")?;
        serializer.serialize_map_value(&mut state, JsonApiObject)?;
        serializer.serialize_map_end(state)
    }

    fn serialize_resource<T>(self, serializer: &mut S, response: ResourceResponse<JsonApiInclude<S>, T>) -> Result<(), S::Error>
    where T: RawFetch + Represent {
        let id = response.resource.id.to_string();
        let self_link = self.link_maker.resource(T::resource_plural(), &id);
        let links = LinkObject {
            self_link: Some(&self_link),
            related_link: None,
        };
        let resource = JsonApiResourceObject {
            resource: &response.resource,
            linker: &self.link_maker,
            id: &id,
            self_link: &self_link,
        };
        if response.includes.is_empty() {
            self.serialize_document(serializer, links, &resource)
        } else {
            let includes = IncludesObject {
                includes: &response.includes,
                linker: &self.link_maker,
            };
            self.serialize_compound_document(serializer, links, &resource, includes)
        }
    }

    fn serialize_collection<T>(self, serializer: &mut S, response: CollectionResponse<JsonApiInclude<S>, T>) -> Result<(), S::Error>
    where T: RawFetch + Represent {
        let self_link = self.link_maker.collection(T::resource_plural());
        let links = LinkObject {
            self_link: Some(&self_link),
            related_link: None,
        };
        let collection = JsonApiCollectionObject {
            resources: &response.resources,
            linker: &self.link_maker,
        };
        if response.includes.is_empty() {
            self.serialize_document(serializer, links, &collection)
        } else {
            let includes = IncludesObject {
                includes: &response.includes,
                linker: &self.link_maker,
            };
            self.serialize_compound_document(serializer, links, &collection, includes)
        }
    }

    fn serialize_rel(self, serializer: &mut S, response: RelResponse<JsonApiInclude<S>>) -> Result<(), S::Error> {
        let self_link = self.link_maker.relationship(&response.resource, &response.id, response.related);
        let related_link = self.link_maker.related_resource(&response.resource, &response.id, response.related);
        let links = LinkObject {
            self_link: Some(&self_link),
            related_link: Some(&related_link),
        };
        match response.rel {
            Relationship::One(ref identifier)   => {
                let linkage = ToOneLinkage(identifier);
                if response.includes.is_empty() {
                    self.serialize_document(serializer, links, &linkage)
                } else {
                    let includes = IncludesObject {
                        includes: &response.includes,
                        linker: &self.link_maker,
                    };
                    self.serialize_compound_document(serializer, links, &linkage, includes)
                }
            }
            Relationship::Many(ref identifiers) => {
                let linkage = ToManyLinkage(identifiers);
                if response.includes.is_empty() {
                    self.serialize_document(serializer, links, &linkage)
                } else {
                    let includes = IncludesObject {
                        includes: &response.includes,
                        linker: &self.link_maker,
                    };
                    self.serialize_compound_document(serializer, links, &linkage, includes)
                }
            }
        }
    }

    fn serialize_err(self, serializer: &mut S, error: Error) -> Result<(), S::Error> {
        let mut state = serializer.serialize_map(Some(2))?;
        serializer.serialize_map_key(&mut state, "errors")?;
        serializer.serialize_map_value(&mut state, ErrorObject {
            status: &error,
        })?;
        serializer.serialize_map_key(&mut state, "jsonapi")?;
        serializer.serialize_map_value(&mut state, JsonApiObject)?;
        serializer.serialize_map_end(state)
    }
}

impl<T, R, S> Presenter<T, R> for JsonApi<R, S>
where
    T: RawFetch + Represent,
    R: Router,
    S: Serializer + WriteAdapter<R::Response>,
{
    type Include = JsonApiInclude<S>;

    fn prepare(field_set: Option<Vec<String>>, link_maker: R::LinkMaker) -> Self {
        JsonApi {
            link_maker: link_maker,
            field_set: field_set,
            _spoopy: PhantomData,
        }
    }

    fn present_resource(self, response: ResourceResponse<Self::Include, T>) -> R::Response {
        self.respond(Status::Ok, |jsonapi, serializer| jsonapi.serialize_resource(serializer, response))
    }

    fn present_collection(self, response: CollectionResponse<Self::Include, T>) -> R::Response {
        self.respond(Status::Ok, |jsonapi, serializer| jsonapi.serialize_collection(serializer, response))
    }

    fn present_rel(self, response: RelResponse<Self::Include>) -> R::Response {
        self.respond(Status::Ok, |jsonapi, serializer| jsonapi.serialize_rel(serializer, response))
    }

    fn present_no_content(self) -> R::Response {
        self.respond(Status::NoContent, |_, _| Ok(()))
    }

    fn present_err(self, error: Error) -> R::Response {
        self.respond(error.status(), |jsonapi, serializer| jsonapi.serialize_err(serializer, error))
    }
}
