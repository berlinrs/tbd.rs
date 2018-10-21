use crate::relation::Relation;

pub trait HasManyRelationShip {
    type Of: Relation;
    type To: Relation;
}

pub trait BelongsToRelationship {
    type Source: Relation;
    type To: Relation;
}