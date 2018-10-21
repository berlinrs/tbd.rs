trait Field {
    type Model;
    type Type;

    fn name() -> &'static str;
    fn get(model: &Self::Model);
    fn set(model: &mut Self::Model, value: Self::Type);
}

trait FieldSet {

}

impl<A> FieldSet for (A) where A: Field {}
impl<A, B> FieldSet for (A, B) where A: Field, B: Field {}
impl<A, B, C> FieldSet for (A, B, C) where A: Field, B: Field, C: Field {}
impl<A, B, C, D> FieldSet for (A, B, C, D) where A: Field, B: Field, C: Field, D: Field {}
