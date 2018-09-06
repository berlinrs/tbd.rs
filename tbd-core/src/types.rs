use futures::prelude::*;

pub trait Gateway {}

pub trait Repository {
    type Gateway: Gateway;

    fn gateway(&self) -> &Self::Gateway;
}

//This must become `ExecuteAll<Relation>`
pub trait ExecuteAll<ReturnType>: Gateway {
    type Error;
    type Stream: Stream<Item = ReturnType>;

    fn execute_query<Q>(&self, q: &Q) -> Self::Stream
        where Q: Query<QueryMarker=All, ReturnType=ReturnType>;
}

pub trait ExecuteOne<ReturnType, Parameters>: Gateway {
    type Error;
    type Future: Future<Output = Option<ReturnType>>;

    fn execute_query<Q>(&self, q: &Q) -> Self::Future
        where Q: Query<QueryMarker=One, ReturnType=ReturnType, Parameters=Parameters>;
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
    type Parameters;

    fn parameters(&self) -> &Self::Parameters;
}

impl<Q> Query for &Q where Q: Query {
    type ReturnType = Q::ReturnType;
    type QueryMarker = Q::QueryMarker;
    type Parameters = Q::Parameters;

    fn parameters(&self) -> &Self::Parameters {
        (*self).parameters()
    }
}

pub trait Stores<R> : Repository {

}

impl<A, B, T> Stores<(A, B)> for T where T: Stores<A> + Stores<B> {} 

pub trait HasManyRelationShip {
    type Of: Relation;
    type To: Relation;
}

pub trait BelongsToRelationship {
    type Source: Relation;
    type To: Relation;
}