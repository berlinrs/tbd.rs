use futures::prelude::*;
use tbd_query::{Query, One, All, SelectFrom, Find};
use tbd_relation::Relation;
use crate::repository::{Repository,Stores};
use tbd_gateway::Gateway;

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


pub trait Execute<Repos, R, ReturnType>
    where R: Relation,
          Repos: Repository,
          Self: Query<ReturnType=ReturnType> {
    
    type FutureType;

    fn execute(&self, repos: &Repos) -> Self::FutureType
        where Repos: Stores<R>;
}

impl<Repos, R, ReturnType> Execute<Repos, R, ReturnType> for SelectFrom<R>
    where R: Relation,
          Repos: Repository,
          Repos::Gateway: ExecuteAll<ReturnType>,
          Self: Query<QueryMarker=All, ReturnType=ReturnType> {

    type FutureType = <<Repos as Repository>::Gateway as ExecuteAll<ReturnType>>::Stream;

    fn execute(&self, repos: &Repos) -> Self::FutureType
        where Repos: Stores<R> {
        ExecuteAll::execute_query(repos.gateway(), &self)
    }
}

impl<Repos, R, ReturnType, PK> Execute<Repos, R, ReturnType> for Find<PK, SelectFrom<R>>
    where R: Relation,
          Repos: Repository,
          Repos::Gateway: ExecuteOne<ReturnType, Self::Parameters>,
          Self: Query<QueryMarker=One, ReturnType=ReturnType> {

    type FutureType = <<Repos as Repository>::Gateway as ExecuteOne<ReturnType, Self::Parameters>>::Future;

    fn execute(&self, repos: &Repos) -> Self::FutureType
        where Repos: Stores<R> {
        ExecuteOne::execute_query(repos.gateway(), &self)
    }
}

