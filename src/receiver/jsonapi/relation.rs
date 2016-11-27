use std::marker::PhantomData;

use serde::de::{Visitor, MapVisitor, SeqVisitor};
use serde::de::impls::VecVisitor;

use api::raw::{Relationship, Identifier};
use api::rel::Relation;
use receiver::jsonapi::ObjectVisitor;
use receiver::jsonapi::identifier::{JsonApiIdentifier, IdentifierVisitor};
use Deserialize;
use Deserializer;

pub struct RelationshipDocument<T>(pub Relationship, PhantomData<T>);

impl<T: Relation> Deserialize for RelationshipDocument<T> {
    fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
        let inner: JsonApiRelationship<T> = deserializer.deserialize_map(ObjectVisitor(PhantomData))?;
        Ok(RelationshipDocument(inner.0, PhantomData))
    }
}

pub struct JsonApiRelationship<T>(pub Relationship, PhantomData<T>);

impl<T: Relation> Deserialize for JsonApiRelationship<T> {
    fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(RelationshipVisitor(PhantomData))
    }
}

struct RelationshipVisitor<T>(PhantomData<T>);

impl<T: Relation> Visitor for RelationshipVisitor<T> {
    type Value = JsonApiRelationship<T>;

    fn visit_unit<E>(&mut self) -> Result<JsonApiRelationship<T>, E> {
        Ok(JsonApiRelationship(Relationship::One(None), PhantomData))
    }

    fn visit_seq<V: SeqVisitor>(&mut self, visitor: V) -> Result<JsonApiRelationship<T>, V::Error> {
        let mut vec_visitor = VecVisitor::new();
        let vector: Vec<JsonApiIdentifier<T::Resource>> = vec_visitor.visit_seq(visitor)?;
        let vector = vector.into_iter().map(|x| x.0).collect();
        Ok(JsonApiRelationship(Relationship::Many(vector), PhantomData))
    }

    fn visit_map<V: MapVisitor>(&mut self, visitor: V) -> Result<JsonApiRelationship<T>, V::Error> {
        let identifier = IdentifierVisitor::<T::Resource>(PhantomData).visit_map(visitor)?;
        Ok(JsonApiRelationship(Relationship::One(Some(identifier.0)), PhantomData))
    }
}

pub struct ToOneDocument<T>(pub Option<Identifier>, PhantomData<T>);

impl<T: Relation> Deserialize for ToOneDocument<T> {
    fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
        let inner: JsonApiToOne<T> = deserializer.deserialize_map(ObjectVisitor(PhantomData))?;
        Ok(ToOneDocument(inner.0, PhantomData))
    }
}

pub struct JsonApiToOne<T>(pub Option<Identifier>, PhantomData<T>);

impl<T: Relation> Deserialize for JsonApiToOne<T> {
    fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(ToOneVisitor(PhantomData))
    }
}

struct ToOneVisitor<T>(PhantomData<T>);

impl<T: Relation> Visitor for ToOneVisitor<T> {
    type Value = JsonApiToOne<T>;

    fn visit_unit<E>(&mut self) -> Result<JsonApiToOne<T>, E> {
        Ok(JsonApiToOne(None, PhantomData))
    }


    fn visit_map<V: MapVisitor>(&mut self, visitor: V) -> Result<JsonApiToOne<T>, V::Error> {
        let identifier = IdentifierVisitor::<T::Resource>(PhantomData).visit_map(visitor)?;
        Ok(JsonApiToOne(Some(identifier.0), PhantomData))
    }
}

pub struct ToManyDocument<T>(pub Vec<Identifier>, PhantomData<T>);

impl<T: Relation> Deserialize for ToManyDocument<T> {
    fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
        let inner: JsonApiToMany<T> = deserializer.deserialize_map(ObjectVisitor(PhantomData))?;
        Ok(ToManyDocument(inner.0, PhantomData))
    }
}

pub struct JsonApiToMany<T>(pub Vec<Identifier>, PhantomData<T>);

impl<T: Relation> Deserialize for JsonApiToMany<T> {
    fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(ToManyVisitor(PhantomData))
    }
}

struct ToManyVisitor<T>(PhantomData<T>);

impl<T: Relation> Visitor for ToManyVisitor<T> {
    type Value = JsonApiToMany<T>;

    fn visit_seq<V: SeqVisitor>(&mut self, visitor: V) -> Result<JsonApiToMany<T>, V::Error> {
        let mut vec_visitor = VecVisitor::new();
        let vector: Vec<JsonApiIdentifier<T::Resource>> = vec_visitor.visit_seq(visitor)?;
        let vector = vector.into_iter().map(|x| x.0).collect();
        Ok(JsonApiToMany(vector, PhantomData))
    }
}
