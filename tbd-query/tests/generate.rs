#![feature(specialization)]

mod schema;

use crate::schema::*;

use tbd_gateway::*;
use tbd_gateway::transaction::*;
use tbd_model_wrappers::Wrapper;
use tbd_relation::Relation;
use tbd_query::compile::*;
use tbd_fieldset::*;

use tbd_query::*;

struct GenericSqlGateway;

struct GenericSqlGatewayTransactionImplementation;

impl TransactionImplementation for GenericSqlGatewayTransactionImplementation {
    fn insert<R>(&mut self, m: &mut R::Wrapper) where R: Relation {

    }
}

impl Gateway for GenericSqlGateway {
    type TransactionImplementation = GenericSqlGatewayTransactionImplementation;

    fn start_transaction(&self) -> Transaction<Self::TransactionImplementation> {
        Transaction { transaction: GenericSqlGatewayTransactionImplementation }
    }
    fn finish_transaction(&self, transaction: Transaction<Self::TransactionImplementation>) {

    }
}

impl<F, R> Compile<SelectFrom<F, R>, Complete> for GenericSqlGateway where R: Relation, F: FieldSet<Model=R::Model, Marker=Complete> {

    fn compile(query: &SelectFrom<F, R>) -> CompiledQuery {
        format!("SELECT * FROM {}", R::name())
    }
}

impl<F, R> Compile<SelectFrom<F, R>, Sparse> for GenericSqlGateway where R: Relation, F: FieldSet<Model=R::Model, Marker=Sparse> {
    fn compile(query: &SelectFrom<F, R>) -> CompiledQuery {
        format!("SELECT * FROM {}", R::name())
    }
}

#[test]
fn simple_select() {
    let query = select::<Post>().from::<Posts>();
    let result = GenericSqlGateway::compile(&query);
    assert_eq!("SELECT * FROM posts", result);
}

#[test]
fn subfield_select() {
    let query = select::<(ContentField)>().from::<Posts>();
    let result = GenericSqlGateway::compile(&query);
    assert_eq!("SELECT content FROM posts", result);
}
