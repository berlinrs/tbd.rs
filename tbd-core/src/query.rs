use crate::execute::*;
use crate::relation::Relation;
use crate::repository::{Stores, Repository};

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

pub struct Limit<Q> {
    max: usize,
    inner: Q,
}

pub struct FindParameters<PK> {
    pub id: PK
}

pub struct Find<PK, Q> {
    params: FindParameters<PK>,
    query: Limit<Q>,
}

impl<PK, Q> Find<PK, Q> {
    fn new(id: PK, query: Q) -> Self {
        Find { params: FindParameters { id: id }, query: Limit { max: 1, inner: query }}
    }
}

impl<PK, Q> Query for Find<PK, Q> where Q: Query {
    type ReturnType = Q::ReturnType;
    type QueryMarker = One;
    type Parameters = FindParameters<PK>;

    fn parameters(&self) -> &FindParameters<PK> {
        &self.params
    }
}

pub struct Select<M> {
    m: std::marker::PhantomData<M>,
}

impl<M> Query for Select<M> {
    type ReturnType = M;
    type QueryMarker = Incomplete;
    type Parameters = ();

    fn parameters(&self) -> &() {
        &()
    }
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

    type Parameters = ();

    fn parameters(&self) -> &() {
        &()
    }
}

impl<R> SelectFrom<R> 
    where R: Relation {

    pub fn limit(self, max: usize) -> Limit<Self> {
        Limit { max: max, inner: self } // why should I?
    }

    pub fn find(self, id: R::PrimaryKey) -> Find<R::PrimaryKey, Self> {
        Find::new(id, self)
    }
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

