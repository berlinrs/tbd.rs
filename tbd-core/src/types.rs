use futures::prelude::*;

pub trait Finite {}

pub trait Repository {}

pub trait Gateway {}

pub trait Relation {
    type PrimaryKey;
    type Model;
}

pub trait SqlRepos {}

pub trait Query {}

pub trait PostGres: SqlRepos {}

pub trait PostGresQuery: Query {}

pub trait Stores<R: Relation>: Repository {
    type Error;

    type Stream: Stream<Item = R::Model>;
    type Future: Future<Output = Option<R::Model>>;

    fn all(&self) -> Self::Stream;

    fn one(&self, id: R::PrimaryKey) -> Self::Future;
}

trait Contains<R> {}

impl<T, R> Contains<R> for T where T: Stores<R>, R: Relation {}

impl<A, B, T> Contains<(A, B)> for T where T: Contains<A> + Contains<B> {} 

pub trait HasManyRelationShip {
    type Of: Relation;
    type To: Relation;
}

pub trait BelongsToRelationship {
    type Source: Relation;
    type To: Relation;
}