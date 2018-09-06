use futures::prelude::*;

pub trait Finite {}

pub trait Gateway {}

pub trait Repository {
    type Gateway: Gateway;

    fn gateway(&self) -> &Self::Gateway;
}

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