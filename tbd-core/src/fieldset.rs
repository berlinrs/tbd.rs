pub trait FieldType {}

pub trait Field {
    type Model;
    type Type: FieldType;

    fn name() -> &'static str;
    fn get(model: &Self::Model) -> &Self::Type;
    fn get_mut(model: &mut Self::Model) -> &mut Self::Type;
}

pub trait FieldSet {
    type Model;
}

impl<A, M> FieldSet for (A) where A: Field<Model = M> {
    type Model = M;
}

impl<A, B, M> FieldSet for (A, B) where A: Field<Model = M>, B: Field<Model = M> {
    type Model = M;
}
// impl<A, B, C, M> FieldSet<T, Model = M> for (A, B, C) where A: Field<Model = M>, B: Field<Model = M>, C: Field<Model = M> {}
// impl<A, B, C, D, M> FieldSet<T, Model = M> for (A, B, C, D) where A: Field<Model = M>, B: Field<Model = M>, C: Field<Model = M>, D: Field<Model = M> {}

// pub trait Apply<F: FieldSet, T: FieldSet> {
// }

pub trait ToModel<F: FieldSet> {
    fn create(self) -> F::Model;
}

impl FieldType for String {}
impl<T> FieldType for Option<T> where T: FieldType {};