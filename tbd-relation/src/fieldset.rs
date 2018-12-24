use tbd_fieldset::*;
use crate::Relation;

pub trait RelationField: Field {
    type Relation;

    fn name() -> &'static str;
}

pub trait RelationFieldSet: FieldSet {
    type Relation;

    fn names() -> Vec<&'static str>;
}


impl<A, R> RelationFieldSet for (A) where A: RelationField<Relation=R>, R: Relation {
    type Relation = R;

    fn names() -> Vec<&'static str> {
        let mut vec = Vec::new();
        vec.push(A::name());
        vec
    }
}

impl<A, B, R> RelationFieldSet for (A, B) where A: RelationField<Relation=R>, B: RelationField<Relation=R>, R: Relation {
    type Relation = R;

    fn names() -> Vec<&'static str> {
        let mut vec = Vec::new();
        vec.push(A::name());
        vec.push(B::name());
        vec
    }
}
