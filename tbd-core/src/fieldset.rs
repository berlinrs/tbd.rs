trait Field {
    type Model;
    type Type;

    fn name() -> &'static str;
    fn get(model: &M);
    fn set(model: &mut M, value: T);
}

trait FieldSet {

}

impl FieldSet for (A) where A: Field {}
impl FieldSet for (A, B) where A: Field, B: Field {}
impl FieldSet for (A, B, C) where A: Field, B: Field, C: Field {}
impl FieldSet for (A, B, C, D) where A: Field, B: Field, C: Field, D: Field {}
