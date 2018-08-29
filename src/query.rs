use crate::types::*;
use futures::prelude::*;

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