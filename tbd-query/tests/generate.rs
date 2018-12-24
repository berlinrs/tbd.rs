#![feature(specialization)]

mod schema;

use crate::schema::*;

use std::fmt::Display;

use tbd_gateway::*;
use tbd_gateway::transaction::*;
use tbd_model_wrappers::Wrapper;
use tbd_relation::Relation;
use tbd_relation::fieldset::*;

use tbd_query::compile::*;
use tbd_fieldset::*;

use tbd_query::*;

struct GenericSqlGateway;

struct GenericSqlGatewayTransactionImplementation;

impl TransactionImplementation for GenericSqlGatewayTransactionImplementation {
    fn insert<R>(&mut self, m: &mut R::Fields) where R: Relation {

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

impl<F, R> Compile<SelectFrom<F, R>, Complete> for GenericSqlGateway where R: Relation, F: RelationFieldSet<Relation=R> + FieldSet<Marker=Complete> {
    fn compile(_query: &SelectFrom<F, R>) -> CompiledQuery {
        format!("SELECT * FROM {}", R::name())
    }
}

impl<F, R> Compile<SelectFrom<F, R>, Sparse> for GenericSqlGateway where R: Relation,  F: RelationFieldSet<Relation=R> + FieldSet<Marker=Sparse> {
    fn compile(_query: &SelectFrom<F, R>) -> CompiledQuery {
        format!("SELECT {} FROM {}", F::names().join(","), R::name())
    }
}

impl<Q, M> Compile<Limit<Q>, M> for GenericSqlGateway where Q: Query + CompileFor<GenericSqlGateway, M>, M: FieldSetMarker {
    fn compile(query: &Limit<Q>) -> CompiledQuery {
        format!("{} LIMIT {}", query.inner().compile(), query.max())
    }
}

// TODO: The "Display" bound on the PK should probably become a ToSql one
impl<F, Q, M> Compile<Find<F, Q>, M> for GenericSqlGateway where Q: Query + CompileFor<GenericSqlGateway, M>, F: PrimaryField + RelationField + FieldSet<Marker=M>, M: FieldSetMarker, F::Type: Display {
    fn compile(query: &Find<F, Q>) -> CompiledQuery {
        format!("{} WHERE {} = {} LIMIT 1", query.inner().compile(), F::name(), query.parameter())
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

#[test]
fn simple_select_limit() {
    let query = select::<Post>().from::<Posts>().limit(1);
    let result = GenericSqlGateway::compile(&query);
    assert_eq!("SELECT * FROM posts LIMIT 1", result);
}

#[test]
fn subfield_select_limit() {
    let query = select::<(ContentField)>().from::<Posts>().limit(1);
    let result = GenericSqlGateway::compile(&query);
    assert_eq!("SELECT content FROM posts LIMIT 1", result);
}

#[test]
fn simple_select_find() {
    let query = select::<Post>().from::<Posts>().limit(1);
    let result = GenericSqlGateway::compile(&query);
    assert_eq!("SELECT * FROM posts LIMIT 1", result);
}

#[test]
fn subfield_select_find() {
    let query = select::<(ContentField)>().from::<Posts>().find(1);
    let result = GenericSqlGateway::compile(&query);
    assert_eq!("SELECT content FROM posts WHERE id = 1 LIMIT 1", result);
}

