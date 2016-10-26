use std::collections::{BTreeMap, HashMap};
use std::collections::btree_map::Iter as BTreeMapIter;
use std::collections::hash_map::Iter as HashMapIter;
use std::iter::{self, Empty, Map};

use api::Error;
use api::raw::identifier::Identifier;
use BASE_URL;
use links::{LinkObject, make_link};
use Serialize;
use Serializer;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Relationship {
    One(Option<Identifier>),
    Many(Vec<Identifier>),
}

pub trait FetchRelationships<'a>: 'a {
    type Iter: Iterator<Item = (&'a str, &'a Relationship)>;
    fn iter(&'a self) -> Self::Iter;
    fn count(&self) -> usize;
}

pub trait UpdateRelationships: Sized {
    fn from_iter<I>(iter: I) -> Result<Self, Error> where I: Iterator<Item = (String, Relationship)>;
}

impl<'a> FetchRelationships<'a> for () {
    type Iter = Empty<(&'a str, &'a Relationship)>;

    fn iter(&'a self) -> Self::Iter {
        iter::empty()
    }

    fn count(&self) -> usize {
        0
    }
}

impl UpdateRelationships for () {
    fn from_iter<I>(mut iter: I) -> Result<Self, Error> where I: Iterator<Item = (String, Relationship)> {
        if let None = iter.next() {
            Ok(())
        } else {
            Err(Error::Conflict)
        }
    }
}

impl<'a> FetchRelationships<'a> for BTreeMap<&'static str, Relationship> {
    type Iter = Map<BTreeMapIter<'a, &'static str, Relationship>,
                    fn((&'a &'static str, &'a Relationship)) -> (&'a str, &'a Relationship)>;

    fn iter(&'a self) -> Self::Iter {
        self.iter().map(deref_str)
    }

    fn count(&self) -> usize {
        self.len()
    }
}

impl<'a> FetchRelationships<'a> for BTreeMap<String, Relationship> {
    type Iter = Map<BTreeMapIter<'a, String, Relationship>,
                    fn((&'a String, &'a Relationship)) -> (&'a str, &'a Relationship)>;

    fn iter(&'a self) -> Self::Iter {
        self.iter().map(deref_string)
    }

    fn count(&self) -> usize {
        self.len()
    }
}

impl UpdateRelationships for BTreeMap<String, Relationship> {
    fn from_iter<I>(iter: I) -> Result<Self, Error> where I: Iterator<Item = (String, Relationship)> {
        Ok(iter.collect())
    }
}

impl<'a> FetchRelationships<'a> for HashMap<&'static str, Relationship> {
    type Iter = Map<HashMapIter<'a, &'static str, Relationship>,
                    fn((&'a &'static str, &'a Relationship)) -> (&'a str, &'a Relationship)>;

    fn iter(&'a self) -> Self::Iter {
        self.iter().map(deref_str)
    }

    fn count(&self) -> usize {
        self.len()
    }
}

impl<'a> FetchRelationships<'a> for HashMap<String, Relationship> {
    type Iter = Map<HashMapIter<'a, String, Relationship>,
                    fn((&'a String, &'a Relationship)) -> (&'a str, &'a Relationship)>;

    fn iter(&'a self) -> Self::Iter {
        self.iter().map(deref_string)
    }

    fn count(&self) -> usize {
        self.len()
    }
}

impl UpdateRelationships for HashMap<String, Relationship> {
    fn from_iter<I>(iter: I) -> Result<Self, Error> where I: Iterator<Item = (String, Relationship)> {
        Ok(iter.collect())
    }
}

fn deref_str<'a>((&key, val): (&'a &'static str, &'a Relationship)) -> (&'a str, &'a Relationship) {
    (key, val)
}

fn deref_string<'a>((key, val): (&'a String, &'a Relationship)) -> (&'a str, &'a Relationship) {
    (&key[..], val)
}

pub struct SerializeRelationships<'a, R: FetchRelationships<'a>> {
    pub resource: &'static str,
    pub id: &'a str,
    pub relationships: &'a R
}

impl<'a, R> Serialize for SerializeRelationships<'a, R> where R: FetchRelationships<'a> {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let mut state = serializer.serialize_map(Some(self.relationships.count()))?;
        for (name, relationship) in self.relationships.iter() {
            serializer.serialize_map_key(&mut state, name)?;
            serializer.serialize_map_value(&mut state, SerializeRelationship {
                base_resource: self.resource,
                base_id: self.id,
                relation: name,
                relationship: relationship,
            })?;
        }
        serializer.serialize_map_end(state)
    }
}

struct SerializeRelationship<'a> {
    base_resource: &'static str,
    base_id: &'a str,
    relation: &'a str,
    relationship: &'a Relationship,
}

impl<'a> Serialize for SerializeRelationship<'a> {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let mut state = serializer.serialize_map(Some(2))?;
        serializer.serialize_map_key(&mut state, "links")?;
        serializer.serialize_map_value(&mut state, LinkObject {
            self_link: Some(&make_link(&[
                BASE_URL,
                self.base_resource,
                self.base_id,
                "relationships",
                self.relation,
            ])),
            related_link: Some(&make_link(&[
                BASE_URL,
                self.base_resource,
                self.base_id,
                self.relation,
            ])),
        })?;
        serializer.serialize_map_key(&mut state, "data")?;
        match *self.relationship {
            Relationship::One(ref identifier)   => {
                serializer.serialize_map_value(&mut state, identifier)?;
            }
            Relationship::Many(ref identifiers) => {
                serializer.serialize_map_value(&mut state, identifiers)?;
            }
        }
        serializer.serialize_map_end(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use api::raw::Identifier;
    use std::collections::BTreeMap;
    use to_value;
    use Value;

    #[test]
    fn serialize_rel_to_one_empty() {
        let rel = super::SerializeRelationship {
            base_resource: "base",
            base_id: "1",
            relation: "relation",
            relationship: &Relationship::One(None),
        };
        let expected = {
            let mut relationship = BTreeMap::new();
            let mut links = BTreeMap::new();
            links.insert(String::from("self"), to_value("https://example.org/api/base/1/relationships/relation"));
            links.insert(String::from("related"), to_value("https://example.org/api/base/1/relation"));
            relationship.insert(String::from("links"), Value::Object(links));
            relationship.insert(String::from("data"), to_value(()));
            Value::Object(relationship)
        };
        assert_eq!(to_value(rel), expected);
    }

    #[test]
    fn serialize_rel_to_one_some() {
        let rel = super::SerializeRelationship {
            base_resource: "base",
            base_id: "1",
            relation: "relation",
            relationship: &Relationship::One(Some(Identifier {
                resource: "related",
                id: String::from("2"),
            })),
        };
        let expected = {
            let mut relationship = BTreeMap::new();
            let mut links = BTreeMap::new();
            links.insert(String::from("self"), to_value("https://example.org/api/base/1/relationships/relation"));
            links.insert(String::from("related"), to_value("https://example.org/api/base/1/relation"));
            relationship.insert(String::from("links"), Value::Object(links));
            let mut identifier = BTreeMap::new();
            identifier.insert(String::from("type"), to_value("related"));
            identifier.insert(String::from("id"), to_value("2"));
            relationship.insert(String::from("data"), to_value(identifier));
            Value::Object(relationship)
        };
        assert_eq!(to_value(rel), expected);
    }

    #[test]
    fn serialize_rel_to_many_empty() {
        let rel = super::SerializeRelationship {
            base_resource: "base",
            base_id: "1",
            relation: "relation",
            relationship: &Relationship::Many(vec![]),
        };
        let expected = {
            let mut relationship = BTreeMap::new();
            let mut links = BTreeMap::new();
            links.insert(String::from("self"), to_value("https://example.org/api/base/1/relationships/relation"));
            links.insert(String::from("related"), to_value("https://example.org/api/base/1/relation"));
            relationship.insert(String::from("links"), Value::Object(links));
            relationship.insert(String::from("data"), to_value(Vec::<Value>::new()));
            Value::Object(relationship)
        };
        assert_eq!(to_value(rel), expected);
    }

    #[test]
    fn serialize_rel_to_many_some() {
        let rel = super::SerializeRelationship {
            base_resource: "base",
            base_id: "1",
            relation: "relation",
            relationship: &Relationship::Many(vec![Identifier {
                resource: "related",
                id: String::from("2"),
            }]),
        };
        let expected = {
            let mut relationship = BTreeMap::new();
            let mut links = BTreeMap::new();
            links.insert(String::from("self"), to_value("https://example.org/api/base/1/relationships/relation"));
            links.insert(String::from("related"), to_value("https://example.org/api/base/1/relation"));
            relationship.insert(String::from("links"), Value::Object(links));
            let mut identifier = BTreeMap::new();
            identifier.insert(String::from("type"), to_value("related"));
            identifier.insert(String::from("id"), to_value("2"));
            relationship.insert(String::from("data"), to_value(vec![identifier]));
            Value::Object(relationship)
        };
        assert_eq!(to_value(rel), expected);
    }
}
