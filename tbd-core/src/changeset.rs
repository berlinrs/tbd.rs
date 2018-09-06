use crate::types::*;

pub trait Changes 
    where Self: Repository + Sized {
    fn change() -> Changeset<Self>;
}

pub trait Changeable<R> where R: Repository,
                              Self: Sized {
    fn insert<Rel>(self, m: Rel::Model) -> Change<Self, R, Insert<Rel>>
        where R: Stores<Rel>,
              Rel: Relation;

    fn commit(&self, repos: &R);
}

pub trait Operation {}

impl<R,G> Changes for R
    where R: Repository<Gateway=G> + Sized,
          G: Gateway {
    fn change() -> Changeset<Self> {
        Changeset { relation: std::marker::PhantomData }
    }
}

pub struct Insert<R: Relation> {
    insert: R::Model
}

impl<R> Operation for Insert<R> where R: Relation {}

pub struct Changeset<R: Repository> {
    relation: std::marker::PhantomData<R>
}

impl<R> Changeable<R> for Changeset<R> where R: Repository {
    fn insert<Rel>(self, m: Rel::Model) -> Change<Self, R, Insert<Rel>>
        where R: Stores<Rel>,
              Rel: Relation {
        Change { after: self, operation: Insert { insert: m }, marker: std::marker::PhantomData }
    }

    fn commit(&self, repos: &R) {
        println!("actually not commiting anything");
    }
}

pub struct Change<C, R, O> where C: Changeable<R>,
                                 R: Repository,
                                 O: Operation {
    pub after: C,
    pub operation: O,
    marker: std::marker::PhantomData<R>
}

impl<C, R, O> Changeable<R> for Change<C, R, O> 
    where C: Changeable<R>,
          R: Repository,
          O: Operation {

    fn insert<Rel>(self, m: Rel::Model) -> Change<Self, R, Insert<Rel>>
        where R: Stores<Rel>,
              Rel: Relation {
        Change { after: self, operation: Insert { insert: m }, marker: std::marker::PhantomData }
    }

    fn commit(&self, repos: &R) {
        println!("actually not commiting anything");
    }
}