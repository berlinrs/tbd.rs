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

impl<R> Execute for SelectFrom<R>
    where R: Relation {
    type R = R;

    fn execute<Repos>(&self, repos: &Repos) -> <Repos as Stores<Self::R>>::Stream
        where Repos: Stores<Self::R> {
        repos.all()
    }
}

pub trait Execute {
    type R: Relation;

    fn execute<Repos>(&self, repos: &Repos) -> <Repos as Stores<Self::R>>::Stream
        where Repos: Stores<Self::R>;
}