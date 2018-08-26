use futures::prelude::*;

pub trait Finite {}

pub trait Repository {}

pub trait Gateway {}

pub trait Relation<Repo: Repository> {
    type PrimaryKey;
    type Model;
    type Error;

    type Stream: Stream<Item = Self::Model>;
    type Future: Future<Output = Option<Self::Model>>;

    fn all(&self, repo: &Repo) -> Self::Stream;
    
    fn one(&self, id: Self::PrimaryKey, repo: &Repo) -> Self::Future;
}

pub trait HasManyRelationShip<RepoOf: Repository, RepoTo: Repository> {
    type Of: Relation<RepoOf>;
    type To: Relation<RepoTo>;
}

pub trait BelongsToRelationship<RepoSource: Repository, RepoTo: Repository> {
    type Source: Relation<RepoSource>;
    type To: Relation<RepoTo>;
}