use std::marker::PhantomData;
use serde::de::{self, Visitor, MapVisitor};

use api::raw::{RawResource, RawReceived, UpdateRelationships, RelationshipError};
use receiver::jsonapi::ObjectVisitor;
use receiver::jsonapi::relation::RelationshipDocument;
use Deserialize;
use Deserializer;

pub struct ResourceDocument<T: RawResource, A: Deserialize>(pub RawReceived<T, A>);

impl<T, A> Deserialize for ResourceDocument<T, A>
where
    T: RawResource,
    A: Deserialize,
{
    fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
        let inner: JsonApiResource<T, A> = deserializer.deserialize_map(ObjectVisitor(PhantomData))?;
        Ok(ResourceDocument(inner.0))
    }
}

pub struct JsonApiResource<T: RawResource, A: Deserialize>(pub RawReceived<T, A>);

impl<T, A> Deserialize for JsonApiResource<T, A>
where
    T: RawResource,
    A: Deserialize,
{
    fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(ResourceVisitor(PhantomData))
    }
}

pub struct ResourceVisitor<T, A>(pub PhantomData<(T, A)>);

impl<T, A> Visitor for ResourceVisitor<T, A>
where
    T: RawResource,
    A: Deserialize,
{
    type Value = JsonApiResource<T, A>;

    fn visit_map<V: MapVisitor>(&mut self, mut visitor: V) -> Result<JsonApiResource<T, A>, V::Error> {
        macro_rules! visit {
            ($visitor:expr => $patterns:tt) => {
                if let Some(key) = visitor.visit_key::<String>()? {
                    match &key[..] $patterns
                } else { return Err(de::Error::custom("Incomplete resource object returned.")) }
            };
        }

        visit!(visitor => {
            "type"          => {
                if visit_resource_type::<T, _>(&mut visitor)? {
                    visit!(visitor => {
                        "attributes"    => {
                            let attributes = visitor.visit_value::<A>()?;
                            visit!(visitor => {
                                "relationships" => {
                                    let relationships = visit_rels::<T, _>(&mut visitor)?;
                                    Ok(JsonApiResource(RawReceived {
                                        attributes: attributes,
                                        relationships: relationships,
                                    }))
                                }
                                otherwise       => Err(de::Error::invalid_value(otherwise)),
                            })
                        }
                        "relationships" => {
                            let relationships = visit_rels::<T, _>(&mut visitor)?;
                            visit!(visitor => {
                                "attributes"    => {
                                    let attributes = visitor.visit_value::<A>()?;
                                    Ok(JsonApiResource(RawReceived {
                                        attributes: attributes,
                                        relationships: relationships,
                                    }))
                                }
                                otherwise       => Err(de::Error::invalid_value(otherwise)),
                            })
                        }
                        otherwise       => Err(de::Error::invalid_value(otherwise)),
                    })
                } else { Err(de::Error::custom("Not correct resource type")) }
            }
            "attributes"    => {
                let attributes = visitor.visit_value::<A>()?;
                visit!(visitor => {
                    "type"          => {
                        if visit_resource_type::<T, _>(&mut visitor)? {
                            visit!(visitor => {
                                "relationships" => {
                                    let relationships = visit_rels::<T, _>(&mut visitor)?;
                                    Ok(JsonApiResource(RawReceived {
                                        attributes: attributes,
                                        relationships: relationships,
                                    }))
                                }
                                otherwise       => Err(de::Error::invalid_value(otherwise)),
                            })
                        } else { Err(de::Error::custom("Not correct resource type")) }
                    }
                    "relationships" => {
                        let relationships = visit_rels::<T, _>(&mut visitor)?;
                        visit!(visitor => {
                            "type"          => {
                                if visit_resource_type::<T, _>(&mut visitor)? {
                                    Ok(JsonApiResource(RawReceived {
                                        attributes: attributes,
                                        relationships: relationships,
                                    }))
                                } else { Err(de::Error::custom("Not correct resource type")) }
                            }
                            otherwise       => Err(de::Error::invalid_value(otherwise)),
                        })
                    }
                    otherwise       => Err(de::Error::invalid_value(otherwise)),
                })
            }
            "relationships" => {
                let relationships = visit_rels::<T, _>(&mut visitor)?;
                visit!(visitor => {
                    "type"          => {
                        if visit_resource_type::<T, _>(&mut visitor)? {
                            visit!(visitor => {
                                "attributes"    => {
                                    let attributes = visitor.visit_value::<A>()?;
                                    Ok(JsonApiResource(RawReceived {
                                        attributes: attributes,
                                        relationships: relationships,
                                    }))
                                }
                                otherwise       => Err(de::Error::invalid_value(otherwise)),
                            })
                        } else { Err(de::Error::custom("Not correct resource type")) }
                    }
                    "attributes"    => {
                        let attributes = visitor.visit_value::<A>()?;
                        visit!(visitor => {
                            "type"          => {
                                if visit_resource_type::<T, _>(&mut visitor)? {
                                    Ok(JsonApiResource(RawReceived {
                                        attributes: attributes,
                                        relationships: relationships,
                                    }))
                                } else { Err(de::Error::custom("Not correct resource type")) }
                            }
                            otherwise       => Err(de::Error::invalid_value(otherwise)),
                        })
                    }
                    otherwise       => Err(de::Error::invalid_value(otherwise)),
                })
            }
            otherwise       => Err(de::Error::invalid_value(otherwise)),
        })
    }
}

fn visit_resource_type<T: RawResource, V: MapVisitor>(visitor: &mut V) -> Result<bool, V::Error> {
    visitor.visit_value::<String>().map(|resource_type| resource_type == T::resource())
}

fn visit_rels<T: RawResource, V: MapVisitor>(visitor: &mut V) -> Result<T::UpdateRels, V::Error> {
    visitor.visit_value::<JsonApiRelationships<T>>().map(|rels| rels.0)
}

struct JsonApiRelationships<T: RawResource>(T::UpdateRels);

impl<T: RawResource> Deserialize for JsonApiRelationships<T> {
    fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(RelationshipsVisitor(PhantomData))
    }
}

struct RelationshipsVisitor<T: RawResource>(PhantomData<T>);

impl<T: RawResource> Visitor for RelationshipsVisitor<T> {
    type Value = JsonApiRelationships<T>;

    fn visit_map<V: MapVisitor>(&mut self, mut visitor: V) -> Result<JsonApiRelationships<T>, V::Error> {
        let mut rels = <T as RawResource>::UpdateRels::default();
        while let Some(key) = visitor.visit_key::<String>()? {
            let relationship = visitor.visit_value::<RelationshipDocument<()>>()?;
            match rels.add_relationship(key, relationship.0) {
                Ok(())                                          => continue,
                Err(RelationshipError::NoSuchRelationship)      => return Err(de::Error::invalid_value("no such relationship")),
                Err(RelationshipError::RelationshipAddedTwice)  => return Err(de::Error::invalid_value("relationship added twice")),
            }
        }
        Ok(JsonApiRelationships(rels))
    }
}
