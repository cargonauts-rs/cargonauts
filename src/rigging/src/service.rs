use std::io;
use std::marker::PhantomData;

use futures::BoxFuture;
use futures::stream::BoxStream;

use tokio::{Service, NewService};
use tokio::stream::{StreamService, NewStreamService};

use mainsail::{Get, Index, ResourceEndpoint, Error};
use mainsail::relations::{GetOne, GetMany, RelationEndpoint, Relationship};
use request::{GetRequest, IndexRequest};

pub struct GetService<T: Get + ResourceEndpoint> {
    _spoopy: PhantomData<T>,
}

impl<T: Get + ResourceEndpoint> Default for GetService<T> {
    fn default() -> Self {
        Self { _spoopy: PhantomData, }
    }
}

impl<T: Get + ResourceEndpoint> Service for GetService<T> {
    type Request = GetRequest<T>;
    type Response = T;
    type Error = Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        T::get(req.identifier, req.env)
    }
}

impl<T: Get + ResourceEndpoint> NewService for GetService<T> {
    type Request = GetRequest<T>;
    type Response = T;
    type Error = Error;
    type Instance = Self;

    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(Self {
            _spoopy: PhantomData,
        })
    }
}

pub struct IndexService<T: Index + ResourceEndpoint> {
    _spoopy: PhantomData<T>,
}

impl<T: Index + ResourceEndpoint> Default for IndexService<T> {
    fn default() -> Self {
        Self { _spoopy: PhantomData, }
    }
}

impl<T> StreamService for IndexService<T> where
    T: Index + ResourceEndpoint,
{
    type Request = IndexRequest<T>;
    type Response = T;
    type Error = Error;
    type Stream = BoxStream<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Stream {
        T::index(req.env)
    }
}

impl<T> NewStreamService for IndexService<T> where
    T: Index + ResourceEndpoint,
{
    type Request = IndexRequest<T>;
    type Response = T;
    type Error = Error;
    type Instance = Self;

    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(Self {
            _spoopy: PhantomData,
        })
    }
}

pub struct GetOneService<T: GetOne<R> + RelationEndpoint<R>, R: Relationship>
where
    R::Related: ResourceEndpoint,
{
    _spoopy: PhantomData<(T, R)>,
}

impl<T: GetOne<R> + RelationEndpoint<R>, R: Relationship> Default for GetOneService<T, R>
where
    R::Related: ResourceEndpoint,
{
    fn default() -> Self {
        Self { _spoopy: PhantomData, }
    }
}

impl<T: GetOne<R> + RelationEndpoint<R>, R: Relationship> Service for GetOneService<T, R>
where
    R::Related: ResourceEndpoint,
{
    type Request = GetRequest<T>;
    type Response = R::Related;
    type Error = Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        T::get_one(req.identifier, req.env)
    }
}

impl<T: GetOne<R> + RelationEndpoint<R>, R: Relationship> NewService for GetOneService<T, R>
where
    R::Related: ResourceEndpoint,
{
    type Request = GetRequest<T>;
    type Response = R::Related;
    type Error = Error;
    type Instance = Self;

    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(Self {
            _spoopy: PhantomData,
        })
    }
}

pub struct GetManyService<T: GetMany<R> + RelationEndpoint<R>, R: Relationship>
where
    R::Related: ResourceEndpoint,
{
    _spoopy: PhantomData<(T, R)>,
}

impl<T: GetMany<R> + RelationEndpoint<R>, R: Relationship> Default for GetManyService<T, R>
where
    R::Related: ResourceEndpoint,
{
    fn default() -> Self {
        Self { _spoopy: PhantomData, }
    }
}

impl<T: GetMany<R> + RelationEndpoint<R>, R: Relationship> StreamService for GetManyService<T, R>
where
    R::Related: ResourceEndpoint,
{
    type Request = GetRequest<T>;
    type Response = R::Related;
    type Error = Error;
    type Stream = BoxStream<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Stream {
        T::get_many(req.identifier, req.env)
    }
}

impl<T: GetMany<R> + RelationEndpoint<R>, R: Relationship> NewStreamService for GetManyService<T, R>
where
    R::Related: ResourceEndpoint,
{
    type Request = GetRequest<T>;
    type Response = R::Related;
    type Error = Error;
    type Instance = Self;

    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(Self {
            _spoopy: PhantomData,
        })
    }
}
