use crate::types::*;
use crate::types;

use futures::prelude::*;

pub struct Limit<Q> {
    inner: Q
}

pub struct PgRandom<Q> {
    inner: Q
}

/// How do I make sure
/// PGAdapter accepts generic query + PGQuery
/// MySQLAdapter accepts generic query + MySQLQuery
/// but PGAdapter does not accept MySQLQuery?
// impl<Q> Query for PgRandom<Q> {}
// impl<Q> PostGresQuery for PgRandom<Q> {}

// trait PgQueryExtension
//     where Self: Sized {
//     fn random(self) -> PgRandom<Self> {
//         PgRandom { inner: self }
//     }
// }

pub struct Select<M> {
    m: std::marker::PhantomData<M>,
}

impl<M> Query for Select<M> {
    type ReturnType = M;
    type QueryMarker = Incomplete;
}

pub fn select<M>() -> Select<M> {
    Select { m: std::marker::PhantomData }
}

impl<M> Select<M> {
    pub fn from<R>(self) -> SelectFrom<R>
        where R: Relation<Model = M> {
            SelectFrom { relation: std::marker::PhantomData }
    }
}

pub struct SelectFrom<R>
    where R: Relation {
    relation: std::marker::PhantomData<R>,
}

impl<R> Query for SelectFrom<R>
    where R: Relation {
    type ReturnType = R::Model;
    type QueryMarker = All;
}

impl<R> SelectFrom<R> 
    where R: Relation {

    pub fn limit(self) -> Limit<Self> {
        Limit { inner: self } // why should I?
    }
}


pub trait Execute<Repos, R>
    where R: Relation,
          Repos: Repository,
          Self: Query {
    
    type ReturnType;

    fn execute(&self, repos: &Repos) -> <Self as Execute<Repos, R>>::ReturnType
        where Repos: Contains<R>;
}

impl<Repos, R> Execute<Repos, R> for SelectFrom<R>
    where R: Relation,
          Repos: Repository,
          Self: Query<QueryMarker=All, ReturnType=<Repos::Gateway as ExecuteAll>::ReturnType> {

    type ReturnType = <Repos::Gateway as ExecuteAll>::Stream;

    default fn execute(&self, repos: &Repos) -> <Self as Execute<Repos, R>>::ReturnType
        where Repos: Contains<R> {
        types::ExecuteAll::execute_query(repos.gateway(), &self)
    }
}

impl<Repos, R> Execute<Repos, R> for SelectFrom<R>
    where R: Relation,
          Repos: Repository,
          Self: Query<QueryMarker=One, ReturnType=<Repos::Gateway as ExecuteOne>::ReturnType> {

    type ReturnType = <Repos::Gateway as ExecuteOne>::Future;

    default fn execute(&self, repos: &Repos) -> <Self as Execute<Repos, R>>::ReturnType
        where Repos: Contains<R> {
        types::ExecuteOne::execute_query(repos.gateway(), &self)
    }
}


// impl<Repos, R> Execute<Repos, R> for SelectFrom<R>
//     where R: Relation,
//           Repos: Stores<R> + SqlRepos {

//     default fn execute(&self, repos: &Repos) -> <Repos as Stores<R>>::Stream
//         where Repos: Stores<R> {
//         repos.all()
//     }
// }

// impl<Repos, R> Execute<Repos, R> for SelectFrom<R>
//     where R: Relation,
//           Repos: Stores<R> + SqlRepos + PostGres {

//     fn execute(&self, repos: &Repos) -> <Repos as Stores<R>>::Stream
//         where Repos: Stores<R> {
//         repos.all()
//     }
// }
