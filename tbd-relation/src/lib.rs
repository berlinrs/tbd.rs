use tbd_model_wrappers::Wrapper;
use tbd_lifecycle::ModelLifeCycle;
use tbd_key::Key;
use tbd_fieldset::*;

use std::collections::HashMap;

pub trait Relation {
    type PrimaryKey: Key + FieldType;
    type PrimaryField: Field<Type=Self::PrimaryKey> + PrimaryField;
    type Model;
    type Fields: FieldSet<Marker=Complete, Model=Self::Model>;
    type Wrapper: Wrapper<Wrapping=Self::Model> + ModelLifeCycle<PrimaryKey=Self::PrimaryKey>;

    // TODO change this signature, HashMap<String, String> is obviously
    // not what we want
    // TODO this method should be removed altogether, it's not a good mapping
    // protocol
    fn hydrate(model: &Self::Wrapper) -> HashMap<String, String>;

    // this should move to Stores
    fn name() -> &'static str;
}
