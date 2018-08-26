use crate::types::Relation;
use crate::types::Repository;

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

pub struct SelectFrom<'a, Repos: Repository, R: Relation<Repos> + 'a> {
    relation: &'a R,
    phantom: std::marker::PhantomData<Repos>,
}
