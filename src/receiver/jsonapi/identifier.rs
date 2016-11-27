use std::borrow::Cow;
use std::marker::PhantomData;

use serde::de::{self, Visitor, MapVisitor};

use api::Resource;
use api::raw::Identifier;
use receiver::jsonapi::ObjectVisitor;
use Deserialize;
use Deserializer;

pub struct IdentifiersDocument<T>(pub Vec<Identifier>, PhantomData<T>);

impl<T> Deserialize for IdentifiersDocument<T>
where
    T: Resource,
{
    fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
        let inner: Vec<JsonApiIdentifier<T>> = deserializer.deserialize_map(ObjectVisitor(PhantomData))?;
        let identifiers = inner.into_iter().map(|x| x.0).collect();
        Ok(IdentifiersDocument(identifiers, PhantomData))
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

pub struct IdentifierVisitor<T>(pub PhantomData<T>);

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

impl<T: Resource> Visitor for IdentifierVisitor<T> {

    fn visit_map<V: MapVisitor>(&mut self, mut visitor: V) -> Result<JsonApiIdentifier<T>, V::Error> {

        visit!(visitor => {
            "type"      => {
                if visit_resource::<T, _>(&mut visitor)? {
                    visit!(visitor => {
                        "id"        => {
                            let id = visitor.visit_value::<String>()?;
                            Ok(JsonApiIdentifier(Identifier {
                                resource: Cow::Borrowed(T::resource()),
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
                        if visit_resource::<T, _>(&mut visitor)? {
                            Ok(JsonApiIdentifier(Identifier {
                                resource: Cow::Borrowed(T::resource()),
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


fn visit_resource<T: Resource, V: MapVisitor>(visitor: &mut V) -> Result<bool, V::Error> {
    visitor.visit_value::<String>().map(|resource_type| resource_type == T::resource())
}
