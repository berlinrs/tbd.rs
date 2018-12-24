pub mod fieldset;
use crate::fieldset::*;

use tbd_key::Key;
use tbd_fieldset::*;

// TODO Do we need the Model here?
// We can also just put a FieldSet here
// and use AssociatedFieldSet for querying
pub trait Relation where Self: Sized {
    type PrimaryKey: Key + FieldType;
    type PrimaryField: RelationField<Type=Self::PrimaryKey> + PrimaryField;
    type Fields: RelationFieldSet<Marker=Complete, Relation=Self>;

    fn name() -> &'static str;
    //fn fields() -> &Self::Fields;
}
