use tbd_model_wrappers::Wrapper;
use tbd_key::Key;

pub trait ModelLifeCycle : Wrapper {
    type PrimaryKey: Key;

    fn created(&mut self, primary_key: &[u8]);
}