use crate::types::Relation;
use crate::types::Repository;
use futures::prelude::*;

pub struct Select<M> {
    m: std::marker::PhantomData<M>,
}

pub fn select<M>() -> Select<M> {
    Select { m: std::marker::PhantomData }
}

impl<M> Select<M> {
    pub fn from<R, Repos>(self, relation: &R) -> SelectFrom<Repos, R>
        where R: Relation<Repos, Model = M>,
              Repos: Repository {
            SelectFrom { relation, phantom: std::marker::PhantomData }
    }
}

pub struct SelectFrom<'a, Repos, R>
   where Repos: Repository,
         R: Relation<Repos> + 'a {
    relation: &'a R,
    phantom: std::marker::PhantomData<Repos>,
}

impl<'a, Repos, R> SelectFrom<'a, Repos, R>
    where Repos: Repository,
          R: Relation<Repos> + 'a {

    pub fn execute(&self, repos: &Repos) -> impl Stream<Item = R::Model> {
        self.relation.all(repos)
    }
}