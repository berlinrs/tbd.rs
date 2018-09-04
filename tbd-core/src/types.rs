use futures::prelude::*;

pub trait Finite {}

pub trait Gateway {}

pub trait Repository {
    type Gateway: Gateway + ExecuteAll + ExecuteOne;

    fn gateway(&self) -> &Self::Gateway;
}

pub trait ExecuteAll: Gateway {
    type Error;
    type ReturnType;
    type Stream: Stream<Item = Self::ReturnType>;

    fn execute_query<Q>(&self, q: &Q) -> Self::Stream
        where Q: Query<QueryMarker=All, ReturnType=Self::ReturnType>;
}

pub trait ExecuteOne: Gateway {
    type Error;
    type ReturnType;
    type Future: Future<Output = Option<Self::ReturnType>>;

    fn execute_query<Q>(&self, q: &Q) -> Self::Future
        where Q: Query<QueryMarker=One, ReturnType=Self::ReturnType>;
}

pub trait Relation {
    type PrimaryKey;
    type Model;
}

pub trait QueryMarker {}

pub struct One;
pub struct All;
pub struct Incomplete;

impl QueryMarker for One {}
impl QueryMarker for All {}
impl QueryMarker for Incomplete {}

pub trait Query {
    type ReturnType;
    type QueryMarker: QueryMarker;
}

impl<Q> Query for &Q where Q: Query {
    type ReturnType = Q::ReturnType;
    type QueryMarker = Q::QueryMarker;
}

pub trait Stores<R: Relation>: Repository {

    // //TODO: Should become Future<Output = Result<Stream<Item = R::Model>, Self::Error>>
    // type Stream: Stream<Item = R::Model>;
    // //TODO: Should bcome Future<Output = Result<Option<R::Model>, Self::Error>>
    // type Future: Future<Output = Option<R::Model>>;

    // fn all(&self) -> Self::Stream;

    // fn one(&self, id: R::PrimaryKey) -> Self::Future;
}

pub trait Contains<R> {}

impl<T, R> Contains<R> for T where T: Stores<R>, R: Relation {}

impl<A, B, T> Contains<(A, B)> for T where T: Contains<A> + Contains<B> {} 

pub trait HasManyRelationShip {
    type Of: Relation;
    type To: Relation;
}

pub trait BelongsToRelationship {
    type Source: Relation;
    type To: Relation;
}