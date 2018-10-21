use crate::model_wrappers::Wrapper;
use crate::lifecycle::ModelLifeCycle;
use crate::key::Key;

use std::collections::HashMap;

pub trait Relation {
    type PrimaryKey: Key;
    type Model;
    type Wrapper: Wrapper<Wrapping=Self::Model> + ModelLifeCycle<PrimaryKey=Self::PrimaryKey>;

    // TODO change this signature, HashMap<String, String> is obviously
    // not what we want
    // TODO this method should be removed altogether, it's not a good mapping
    // protocol
    fn hydrate(model: &Self::Wrapper) -> HashMap<String, String>;

    // this should move to Stores
    fn name() -> &'static str;
}
