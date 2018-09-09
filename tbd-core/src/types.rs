use futures::prelude::*;
use std::collections::HashMap;
use crate::changeset::TransactionImplementation;
use crate::changeset::Transaction;
use crate::model_wrappers::Wrapper;

pub trait Gateway {
    type TransactionImplementation: TransactionImplementation;

    fn start_transaction(&self) -> Transaction<Self::TransactionImplementation>;
    fn finish_transaction(&self, transaction: Transaction<Self::TransactionImplementation>);
}

pub trait Repository {
    type Gateway: Gateway;

    fn gateway(&self) -> &Self::Gateway;
}

//This must become `ExecuteAll<Relation>`, because we
//probably need the relation name
//Alternatively, funnel the relation name through the query
pub trait ExecuteAll<ReturnType>: Gateway {
    type Error;
    type Stream: Stream<Item = ReturnType>;

    fn execute_query<Q>(&self, q: &Q) -> Self::Stream
        where Q: Query<QueryMarker=All, ReturnType=ReturnType>;
}

//This must become `ExecuteOne<Relation>`, because we
//probably need the relation name
//Alternatively, funnel the relation name through the query
pub trait ExecuteOne<ReturnType, Parameters>: Gateway {
    type Error;
    type Future: Future<Output = Option<ReturnType>>;

    fn execute_query<Q>(&self, q: &Q) -> Self::Future
        where Q: Query<QueryMarker=One, ReturnType=ReturnType, Parameters=Parameters>;
}

pub trait Relation {
    type PrimaryKey;
    type Model;
    type Wrapper: Wrapper<Wrapping=Self::Model>;

    // TODO change this signature, HashMap<String, String> is obviously
    // not what we want
    // TODO this method should be removed altogether, it's not a good mapping
    // protocol
    fn hydrate(model: &Self::Model) -> HashMap<String, String>;

    // this should move to Stores
    fn name() -> &'static str;
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