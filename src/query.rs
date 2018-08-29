use crate::types::Relation;
use crate::types::Repository;
use crate::types::Stores;
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

impl<R> SelectFrom<R>
    where R: Relation {

    pub fn execute<Repos>(&self, repos: &Repos) -> impl Stream<Item = R::Model>
        where Repos: Stores<R> {
        repos.all()
    }
}