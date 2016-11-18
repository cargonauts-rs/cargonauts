use std::collections::{BTreeMap, HashMap};
use std::collections::btree_map::Iter as BTreeMapIter;
use std::collections::hash_map::Iter as HashMapIter;
use std::iter::{self, Empty, Map};

use api::raw::Identifier;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct RelationshipLinkage {
    pub linkage: Option<Relationship>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Relationship {
    One(Option<Identifier>),
    Many(Vec<Identifier>),
}

pub trait FetchRelationships<'a>: IntoIterator<Item = (&'static str, RelationshipLinkage)> + 'a {
    type Iter: Iterator<Item = (&'static str, &'a RelationshipLinkage)>;
    fn iter(&'a self) -> Self::Iter;
    fn count(&self) -> usize;
}

pub trait UpdateRelationships: Default {
    fn add_relationship(&mut self, name: String, rel: Relationship) -> Result<(), RelationshipError>;
}

pub enum RelationshipError {
    NoSuchRelationship,
    RelationshipAddedTwice,
}

#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub struct NoRelationships;

impl IntoIterator for NoRelationships {
    type Item = (&'static str, RelationshipLinkage);
    type IntoIter = Empty<(&'static str, RelationshipLinkage)>;

    fn into_iter(self) -> Self::IntoIter {
        iter::empty()
    }
}

impl<'a> FetchRelationships<'a> for NoRelationships {
    type Iter = Empty<(&'static str, &'a RelationshipLinkage)>;

    fn iter(&'a self) -> Self::Iter {
        iter::empty()
    }

    fn count(&self) -> usize {
        0
    }
}

impl UpdateRelationships for NoRelationships {
    fn add_relationship(&mut self, _: String, _: Relationship) -> Result<(), RelationshipError> {
        Err(RelationshipError::NoSuchRelationship)
    }
}

impl<'a> FetchRelationships<'a> for BTreeMap<&'static str, RelationshipLinkage> {
    type Iter = Map<BTreeMapIter<'a, &'static str, RelationshipLinkage>,
                    fn((&'a &'static str, &'a RelationshipLinkage)) -> (&'static str, &'a RelationshipLinkage)>;

    fn iter(&'a self) -> Self::Iter {
        self.iter().map(deref_str)
    }

    fn count(&self) -> usize {
        self.len()
    }
}

impl UpdateRelationships for BTreeMap<String, Relationship> {
    fn add_relationship(&mut self, name: String, rel: Relationship) -> Result<(), RelationshipError> {
        match self.insert(name, rel) {
            Some(_) => Err(RelationshipError::RelationshipAddedTwice),
            None    => Ok(())
        }
    }
}

impl<'a> FetchRelationships<'a> for HashMap<&'static str, RelationshipLinkage> {
    type Iter = Map<HashMapIter<'a, &'static str, RelationshipLinkage>,
                    fn((&'a &'static str, &'a RelationshipLinkage)) -> (&'static str, &'a RelationshipLinkage)>;

    fn iter(&'a self) -> Self::Iter {
        self.iter().map(deref_str)
    }

    fn count(&self) -> usize {
        self.len()
    }
}

impl UpdateRelationships for HashMap<String, Relationship> {
    fn add_relationship(&mut self, name: String, rel: Relationship) -> Result<(), RelationshipError> {
        match self.insert(name, rel) {
            Some(_) => Err(RelationshipError::RelationshipAddedTwice),
            None    => Ok(())
        }
    }
}

fn deref_str<'a>((&key, val): (&'a &'static str, &'a RelationshipLinkage)) -> (&'static str, &'a RelationshipLinkage) {
    (key, val)
}
