use std::collections::{BTreeMap, HashMap};
use std::collections::btree_map::Iter as BTreeMapIter;
use std::collections::hash_map::Iter as HashMapIter;
use std::iter::{self, Empty, Map};

use api::Error;
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

pub trait FetchRelationships<'a>: 'a {
    type Iter: Iterator<Item = (&'a str, &'a RelationshipLinkage)>;
    fn iter(&'a self) -> Self::Iter;
    fn count(&self) -> usize;
}

pub trait UpdateRelationships: Sized {
    fn from_iter<I>(iter: I) -> Result<Self, Error> where I: Iterator<Item = (String, Relationship)>;
}

impl<'a> FetchRelationships<'a> for () {
    type Iter = Empty<(&'a str, &'a RelationshipLinkage)>;

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

impl<'a> FetchRelationships<'a> for BTreeMap<&'static str, RelationshipLinkage> {
    type Iter = Map<BTreeMapIter<'a, &'static str, RelationshipLinkage>,
                    fn((&'a &'static str, &'a RelationshipLinkage)) -> (&'a str, &'a RelationshipLinkage)>;

    fn iter(&'a self) -> Self::Iter {
        self.iter().map(deref_str)
    }

    fn count(&self) -> usize {
        self.len()
    }
}

impl<'a> FetchRelationships<'a> for BTreeMap<String, RelationshipLinkage> {
    type Iter = Map<BTreeMapIter<'a, String, RelationshipLinkage>,
                    fn((&'a String, &'a RelationshipLinkage)) -> (&'a str, &'a RelationshipLinkage)>;

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

impl<'a> FetchRelationships<'a> for HashMap<&'static str, RelationshipLinkage> {
    type Iter = Map<HashMapIter<'a, &'static str, RelationshipLinkage>,
                    fn((&'a &'static str, &'a RelationshipLinkage)) -> (&'a str, &'a RelationshipLinkage)>;

    fn iter(&'a self) -> Self::Iter {
        self.iter().map(deref_str)
    }

    fn count(&self) -> usize {
        self.len()
    }
}

impl<'a> FetchRelationships<'a> for HashMap<String, RelationshipLinkage> {
    type Iter = Map<HashMapIter<'a, String, RelationshipLinkage>,
                    fn((&'a String, &'a RelationshipLinkage)) -> (&'a str, &'a RelationshipLinkage)>;

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

fn deref_str<'a>((&key, val): (&'a &'static str, &'a RelationshipLinkage)) -> (&'a str, &'a RelationshipLinkage) {
    (key, val)
}

fn deref_string<'a>((key, val): (&'a String, &'a RelationshipLinkage)) -> (&'a str, &'a RelationshipLinkage) {
    (&key[..], val)
}
