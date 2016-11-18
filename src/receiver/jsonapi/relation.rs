use std::borrow::Cow;
use std::marker::PhantomData;

use serde::de::{self, Visitor, MapVisitor, SeqVisitor};
use serde::de::impls::VecVisitor;

use api::Resource;
use api::raw::{Relationship, Identifier};
use api::rel::Relation;
use receiver::jsonapi::ObjectVisitor;
use Deserialize;
use Deserializer;

pub struct RelationshipDocument<T>(pub Relationship, PhantomData<T>);

impl<T> Deserialize for RelationshipDocument<T> {
    fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
        let inner: JsonApiRelationship<T> = deserializer.deserialize_map(ObjectVisitor(PhantomData))?;
        Ok(RelationshipDocument(inner.0, PhantomData))
    }
}

pub struct JsonApiRelationship<T>(pub Relationship, PhantomData<T>);

impl<T> Deserialize for JsonApiRelationship<T> {
    fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(RelationshipVisitor(PhantomData))
    }
}

struct RelationshipVisitor<T>(PhantomData<T>);

impl<T> Visitor for RelationshipVisitor<T> {
    type Value = JsonApiRelationship<T>;

    fn visit_unit<E>(&mut self) -> Result<JsonApiRelationship<T>, E> {
        Ok(JsonApiRelationship(Relationship::One(None), PhantomData))
    }

    fn visit_seq<V: SeqVisitor>(&mut self, visitor: V) -> Result<JsonApiRelationship<T>, V::Error> {
        let mut vec_visitor = VecVisitor::new();
        let vector: Vec<JsonApiIdentifier<T>> = vec_visitor.visit_seq(visitor)?;
        let vector = vector.into_iter().map(|x| x.0).collect();
        Ok(JsonApiRelationship(Relationship::Many(vector), PhantomData))
    }

    fn visit_map<V: MapVisitor>(&mut self, visitor: V) -> Result<JsonApiRelationship<T>, V::Error> {
        let identifier = IdentifierVisitor::<T>(PhantomData).visit_map(visitor)?;
        Ok(JsonApiRelationship(Relationship::One(Some(identifier.0)), PhantomData))
    }
}

pub struct JsonApiIdentifier<T>(pub Identifier, PhantomData<T>);

impl<T> Deserialize for JsonApiIdentifier<T> {
    fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(IdentifierVisitor::<T>(PhantomData))
    }
}

macro_rules! visit {
    ($visitor:expr => $patterns:tt) => {
        if let Some(key) = $visitor.visit_key::<String>()? {
            match &key[..] $patterns
        } else { return Err(de::Error::custom("Incomplete resource object returned.")) }
    };
}

struct IdentifierVisitor<T>(PhantomData<T>);

impl<T> Visitor for IdentifierVisitor<T> {
    type Value = JsonApiIdentifier<T>;
    default fn visit_map<V: MapVisitor>(&mut self, mut visitor: V) -> Result<JsonApiIdentifier<T>, V::Error> {
        visit!(visitor => {
            "type"      => {
                let resource = visitor.visit_value::<String>()?;
                visit!(visitor => {
                    "id"        => {
                        let id = visitor.visit_value::<String>()?;
                        Ok(JsonApiIdentifier(Identifier {
                            resource: Cow::Owned(resource),
                            id: id,
                        }, PhantomData))
                    }
                    otherwise   => Err(de::Error::invalid_value(otherwise)),
                })
            }
            "id"        => {
                let id = visitor.visit_value::<String>()?;
                visit!(visitor => {
                    "type"      => {
                        let resource = visitor.visit_value::<String>()?;
                        Ok(JsonApiIdentifier(Identifier {
                            resource: Cow::Owned(resource),
                            id: id,
                        }, PhantomData))
                    }
                    otherwise   => Err(de::Error::invalid_value(otherwise)),
                })
            }
            otherwise   => Err(de::Error::invalid_value(otherwise)),
        })
    }
}

impl<T: Relation> Visitor for IdentifierVisitor<T> {

    fn visit_map<V: MapVisitor>(&mut self, mut visitor: V) -> Result<JsonApiIdentifier<T>, V::Error> {

        visit!(visitor => {
            "type"      => {
                if visit_rel::<T, _>(&mut visitor)? {
                    visit!(visitor => {
                        "id"        => {
                            let id = visitor.visit_value::<String>()?;
                            Ok(JsonApiIdentifier(Identifier {
                                resource: Cow::Borrowed(T::Resource::resource()),
                                id: id,
                            }, PhantomData))
                        }
                        otherwise   => Err(de::Error::invalid_value(otherwise)),
                    })
                } else { Err(de::Error::custom("Not correct resource type")) }
            }
            "id"        => {
                let id = visitor.visit_value::<String>()?;
                visit!(visitor => {
                    "type"      => {
                        if visit_rel::<T, _>(&mut visitor)? {
                            Ok(JsonApiIdentifier(Identifier {
                                resource: Cow::Borrowed(T::Resource::resource()),
                                id: id,
                            }, PhantomData))
                        } else { Err(de::Error::custom("Not correct resource type")) }
                    }
                    otherwise   => Err(de::Error::invalid_value(otherwise)),
                })
            }
            otherwise   => Err(de::Error::invalid_value(otherwise)),
        })
    }
}

fn visit_rel<T: Relation, V: MapVisitor>(visitor: &mut V) -> Result<bool, V::Error> {
    visitor.visit_value::<String>().map(|resource_type| resource_type == T::Resource::resource())
}
