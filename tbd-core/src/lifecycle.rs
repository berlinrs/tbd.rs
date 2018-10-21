use crate::model_wrappers::Wrapper;
use crate::key::Key;

pub trait ModelLifeCycle : Wrapper {
    type PrimaryKey: Key;

    fn created(&mut self, primary_key: &[u8]);
}