use futures::prelude::*;
use crate::query::{Query, One, All};
use crate::gateway::Gateway;

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
