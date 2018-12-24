use crate::*;
use tbd_gateway::Gateway;
use tbd_fieldset::FieldSetMarker;

pub type CompiledQuery = String;

pub trait Compile<Q, M: FieldSetMarker>: Gateway where Q: Query {
    fn compile(query: &Q) -> CompiledQuery;
}