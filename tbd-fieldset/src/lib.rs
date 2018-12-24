pub trait FieldType {}

pub trait FieldSetMarker {}

pub struct Complete;
pub struct Sparse;

impl FieldSetMarker for Complete {}
impl FieldSetMarker for Sparse {}

pub trait Field {
    type Type: FieldType;
}

/// Marks a primary field
pub trait PrimaryField: Field {

}

/// TODO: Do FieldSets need to be tied to models? no
/// if not, what does the Marker represent? dunno
pub trait FieldSet {
    type Marker: FieldSetMarker;
}

pub trait ModelMappedFieldSet: FieldSet<Marker=Complete> {
    type Model;
}

impl<A> FieldSet for (A) where A: Field {
    type Marker = Sparse;
}

impl<A, B> FieldSet for (A, B) where A: Field, B: Field {
    type Marker = Sparse;
}

pub trait AssociatedFieldSet {
    type Set: FieldSet;
}

impl<F> AssociatedFieldSet for F where F: FieldSet {
    type Set = Self;
}

impl FieldType for String {}
impl FieldType for i64 {}

impl<T> FieldType for Option<T> where T: FieldType {}