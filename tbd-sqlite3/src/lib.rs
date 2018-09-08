#![feature(async_await, await_macro, futures_api, pin, arbitrary_self_types, specialization)]

use rusqlite::Connection;
use tbd_core::types::*;
use tbd_core::query::*;
use tbd_core::changeset::*;
use futures::stream;
use futures::future;

pub struct Sqlite3Gateway {
    pub connection: Connection
}

#[derive(Default, Debug)]
pub struct Sqlite3Transaction {
    statements: Vec<String>
}

impl TransactionImplementation for Sqlite3Transaction {
    fn insert<R>(&mut self, m: &R::Model) where R: Relation {
        let hydrated = R::hydrate(m);

        let (keys, values): (Vec<&str>, Vec<&str>) = 
            hydrated.iter()
                    .map(|(k,v)| (k.as_str(), v.as_str()))
                    .unzip();

        let field_list: String = keys.join(", ");
        let values_list: String = values.join(", ");

        self.statements.push(format!("INSERT INTO {} ({})
                       VALUES ({})", R::name(), field_list, values_list));
    }

}

impl Gateway for Sqlite3Gateway {
    type TransactionImplementation = Sqlite3Transaction;

    fn start_transaction(&self) -> Transaction<Self::TransactionImplementation> {
        Transaction { transaction: Sqlite3Transaction::default() }
    }

    fn execute_transaction(&self, transaction: Transaction<Self::TransactionImplementation>) {
        println!("{:?}", transaction.transaction);
        for stmt in transaction.transaction.statements {
            println!("{}", stmt);
            self.connection.execute(
                &stmt,
                &[],
            ).unwrap();
        }
    }
}

pub trait RelationName {
    fn relation_name() -> &'static str;
}

impl<T> ExecuteAll<T> for Sqlite3Gateway
    where T: RelationName,
          T: for<'a> From<&'a rusqlite::Row<'a, 'a>> {
    type Error = ();
    type Stream = stream::Iter<std::vec::IntoIter<T>>;

    fn execute_query<Q>(&self, q: &Q) -> Self::Stream
        where Q: Query<QueryMarker=All, ReturnType=T> {

        // TODO SECURITY this is unsafe
        let mut stmt = self.connection
            .prepare(&format!("SELECT * FROM {}", T::relation_name()))
            .unwrap();

        let resultvec: Vec<T> = stmt.query_map(&[], |row| {
            T::from(row)
        }).unwrap().map(Result::unwrap).collect();

        stream::iter(resultvec.into_iter())
    }
}


impl<T> ExecuteOne<T, FindParameters<i64>> for Sqlite3Gateway
    where T: RelationName,
          T: for<'a> From<&'a rusqlite::Row<'a, 'a>> {
    type Error = ();
    type Future = futures::future::Ready<Option<T>>;

    fn execute_query<Q>(&self, q: &Q) -> Self::Future
        where Q: Query<QueryMarker=One, ReturnType=T, Parameters=FindParameters<i64>> {

        // TODO SECURITY this is unsafe
        let mut stmt = self.connection
            .prepare(&format!("SELECT * FROM {} where id = {}", T::relation_name(), q.parameters().id))
            .unwrap();

        let mut resultvec: Vec<T> = stmt.query_map(&[], |row| {
            T::from(row)
        }).unwrap().map(Result::unwrap).collect();

        // TODO turn panic into result
        let res = if resultvec.len() > 0 {
            Some(resultvec.remove(0))
        } else {
            None
        };

        future::ready(res)
    }
}