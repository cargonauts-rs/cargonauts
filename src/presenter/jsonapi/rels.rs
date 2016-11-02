use api::raw::{FetchRelationships, RelationshipLinkage, Relationship};
use BASE_URL;
use links::{LinkObject, make_link};
use presenter::jsonapi::linkage::{ToOneLinkage, ToManyLinkage};
use repr::Represent;
use Serialize;
use Serializer;

pub struct RelsObject<'a, R: FetchRelationships<'a>> {
    pub resource: &'static str,
    pub id: &'a str,
    pub relationships: &'a R
}

impl<'a, R> Represent for RelsObject<'a, R> where R: FetchRelationships<'a> {
    fn repr<S: Serializer>(&self, serializer: &mut S, field_set: Option<&[String]>) -> Result<(), S::Error> {
        match field_set {
            Some(field_set) => {
                let mut state = serializer.serialize_map(None)?;
                let relationships = self.relationships.iter().filter(|&(ref name, _)| {
                    field_set.iter().any(|field| field == name)
                });
                for (name, relationship) in relationships {
                    serializer.serialize_map_key(&mut state, name)?;
                    serializer.serialize_map_value(&mut state, ReprRel {
                        base_resource: self.resource,
                        base_id: self.id,
                        relation: name,
                        relationship: relationship,
                    })?;
                }
                serializer.serialize_map_end(state)
            }
            None            => {
                let mut state = serializer.serialize_map(Some(self.relationships.count()))?;
                for (name, relationship) in self.relationships.iter() {
                    serializer.serialize_map_key(&mut state, name)?;
                    serializer.serialize_map_value(&mut state, ReprRel {
                        base_resource: self.resource,
                        base_id: self.id,
                        relation: name,
                        relationship: relationship,
                    })?;
                }
                serializer.serialize_map_end(state)
            }
        }
    }
}

struct ReprRel<'a> {
    base_resource: &'static str,
    base_id: &'a str,
    relation: &'a str,
    relationship: &'a RelationshipLinkage,
}

impl<'a> Serialize for ReprRel<'a> {
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
        match self.relationship.linkage {
            Some(Relationship::One(ref identifier))     => {
                serializer.serialize_map_key(&mut state, "data")?;
                serializer.serialize_map_value(&mut state, ToOneLinkage(identifier))?;
            }
            Some(Relationship::Many(ref identifiers))   => {
                serializer.serialize_map_key(&mut state, "data")?;
                serializer.serialize_map_value(&mut state, ToManyLinkage(identifiers))?;
            }
            None                                        => {}
        }
        serializer.serialize_map_end(state)
    }
}

#[cfg(test)]
mod tests {
    use api::raw::{Identifier, Relationship, RelationshipLinkage};
    use std::collections::BTreeMap;
    use to_value;
    use Value;

    #[test]
    fn serialize_rel_no_linkage() {
        let rel = super::ReprRel {
            base_resource: "base",
            base_id: "1",
            relation: "relation",
            relationship: &RelationshipLinkage::default(),
        };
        let expected = {
            let mut relationship = BTreeMap::new();
            let mut links = BTreeMap::new();
            links.insert(String::from("self"), to_value("https://example.org/api/base/1/relationships/relation"));
            links.insert(String::from("related"), to_value("https://example.org/api/base/1/relation"));
            relationship.insert(String::from("links"), Value::Object(links));
            Value::Object(relationship)
        };
        assert_eq!(to_value(&rel), expected);
    }

    #[test]
    fn serialize_rel_to_one_empty() {
        let rel = super::ReprRel {
            base_resource: "base",
            base_id: "1",
            relation: "relation",
            relationship: &RelationshipLinkage {
                linkage: Some(Relationship::One(None)),
            },
        };
        let expected = {
            let mut relationship = BTreeMap::new();
            let mut links = BTreeMap::new();
            links.insert(String::from("self"), to_value("https://example.org/api/base/1/relationships/relation"));
            links.insert(String::from("related"), to_value("https://example.org/api/base/1/relation"));
            relationship.insert(String::from("links"), Value::Object(links));
            relationship.insert(String::from("data"), to_value(&()));
            Value::Object(relationship)
        };
        assert_eq!(to_value(&rel), expected);
    }

    #[test]
    fn serialize_rel_to_one_some() {
        let rel = super::ReprRel {
            base_resource: "base",
            base_id: "1",
            relation: "relation",
            relationship: &RelationshipLinkage {
                linkage: Some(Relationship::One(Some(Identifier {
                    resource: "related",
                    id: String::from("2"),
                }))),
            },
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
            relationship.insert(String::from("data"), to_value(&identifier));
            Value::Object(relationship)
        };
        assert_eq!(to_value(&rel), expected);
    }

    #[test]
    fn serialize_rel_to_many_empty() {
        let rel = super::ReprRel {
            base_resource: "base",
            base_id: "1",
            relation: "relation",
            relationship: &RelationshipLinkage { linkage: Some(Relationship::Many(vec![])) },
        };
        let expected = {
            let mut relationship = BTreeMap::new();
            let mut links = BTreeMap::new();
            links.insert(String::from("self"), to_value("https://example.org/api/base/1/relationships/relation"));
            links.insert(String::from("related"), to_value("https://example.org/api/base/1/relation"));
            relationship.insert(String::from("links"), Value::Object(links));
            let empty_slice: &[()] = &[];
            relationship.insert(String::from("data"), to_value(empty_slice));
            Value::Object(relationship)
        };
        assert_eq!(to_value(&rel), expected);
    }

    #[test]
    fn serialize_rel_to_many_some() {
        let rel = super::ReprRel {
            base_resource: "base",
            base_id: "1",
            relation: "relation",
            relationship: &RelationshipLinkage {
                linkage: Some(Relationship::Many(vec![Identifier {
                    resource: "related",
                    id: String::from("2"),
                }])),
            },
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
            relationship.insert(String::from("data"), to_value(&[identifier]));
            Value::Object(relationship)
        };
        assert_eq!(to_value(&rel), expected);
    }
}
