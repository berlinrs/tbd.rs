use crate::*;
use tbd_gateway::Gateway;
use tbd_fieldset::FieldSetMarker;

pub type CompiledQuery = String;

pub trait Compile<Q, M: FieldSetMarker>: Gateway where Q: Query {
    fn compile(query: &Q) -> CompiledQuery;
}

pub trait CompileFor<G, M: FieldSetMarker> where G: Gateway {
    fn compile(&self) -> CompiledQuery;
}

impl<G, F, QM, P, Q> CompileFor<G, F::Marker> for Q where G: Gateway,
                                  G: Compile<Self, F::Marker>,
                                  F: FieldSet,
                                  QM: QueryMarker,
                                  Q: Query<ReturnType=F, QueryMarker=QM, Parameters=P> {
    fn compile(&self) -> CompiledQuery {
        G::compile(&self)
    }
}
