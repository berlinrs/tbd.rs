use futures::prelude::*;

pub trait Finite {}

pub trait Repository {}

pub trait Gateway {}

pub trait Relation {
    type PrimaryKey;
    type Model;
}

pub trait SqlRepos {}

pub trait Stores<R: Relation>: Repository {
    type Error;

    type Stream: Stream<Item = R::Model>;
    type Future: Future<Output = Option<R::Model>>;

    fn all(&self) -> Self::Stream;

    fn one(&self, id: R::PrimaryKey) -> Self::Future;
}

pub trait HasManyRelationShip {
    type Of: Relation;
    type To: Relation;
}

pub trait BelongsToRelationship {
    type Source: Relation;
    type To: Relation;
}