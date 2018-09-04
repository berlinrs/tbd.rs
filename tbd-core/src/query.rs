use crate::types::*;
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
impl<Q> Query for PgRandom<Q> {}
impl<Q> PostGresQuery for PgRandom<Q> {}

trait PgQueryExtension
    where Self: Sized {
    fn random(self) -> PgRandom<Self> {
        PgRandom { inner: self }
    }
}

pub struct Select<M> {
    m: std::marker::PhantomData<M>,
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

impl<R> SelectFrom<R> 
    where R: Relation {

    pub fn limit(self) -> Limit<Self> {
        Limit { inner: self } // why should I?
    }
}

impl<Repos, R> Execute<Repos, R> for SelectFrom<R>
    where R: Relation,
          Repos: Stores<R> {

    default fn execute(&self, repos: &Repos) -> <Repos as Stores<R>>::Stream
        where Repos: Stores<R> {
        repos.all()
    }
}

impl<Repos, R> Execute<Repos, R> for SelectFrom<R>
    where R: Relation,
          Repos: Stores<R> + SqlRepos {

    default fn execute(&self, repos: &Repos) -> <Repos as Stores<R>>::Stream
        where Repos: Stores<R> {
        repos.all()
    }
}

impl<Repos, R> Execute<Repos, R> for SelectFrom<R>
    where R: Relation,
          Repos: Stores<R> + SqlRepos + PostGres {

    fn execute(&self, repos: &Repos) -> <Repos as Stores<R>>::Stream
        where Repos: Stores<R> {
        repos.all()
    }
}

pub trait Execute<Repos, R>
    where R: Relation,
          Repos: Stores<R> {

    fn execute(&self, repos: &Repos) -> <Repos as Stores<R>>::Stream
        where Repos: Stores<R>;
}