#![feature(futures_api)]

use rusqlite::Connection;
use tbd_core::changeset::*;
use tbd_gateway::*;
use tbd_gateway::transaction::*;
use tbd_query::*;
use tbd_core::changeset::*;
use tbd_core::execute::*;
use tbd_relation::Relation;
use tbd_lifecycle::ModelLifeCycle;

use futures::stream;
use futures::future;
use std::cell::RefCell;

use byteorder::{WriteBytesExt,LittleEndian};

pub struct Sqlite3Gateway {
    pub connection: RefCell<Option<Connection>>
}

impl Autogenerates<i64> for Sqlite3Gateway {}

pub struct Sqlite3Transaction {
    connection: Connection
}

impl TransactionImplementation for Sqlite3Transaction {
    fn insert<R>(&mut self, m: &mut R::Wrapper) where R: Relation {
        let hydrated = R::hydrate(m);

        let (keys, values): (Vec<&str>, Vec<&str>) = 
            hydrated.iter()
                    .map(|(k,v)| (k.as_str(), v.as_str()))
                    .unzip();

        let field_list: String = keys.join(", ");
        let values_list: String = values.join(", ");

        let stmt = format!("INSERT INTO {} ({})
                       VALUES ({})", R::name(), field_list, values_list);
        
        self.connection.execute(
            &stmt,
            &[],
        ).unwrap();

        let insert_row_id = self.connection.last_insert_rowid();
        println!("inserted {}", insert_row_id);

        let mut wtr = vec![];
        wtr.write_i64::<LittleEndian>(insert_row_id).unwrap();
        m.created(&wtr);
    }
}

impl Gateway for Sqlite3Gateway {
    type TransactionImplementation = Sqlite3Transaction;

    fn start_transaction(&self) -> Transaction<Self::TransactionImplementation> {
        Transaction { transaction: Sqlite3Transaction { connection: self.connection.borrow_mut().take().unwrap() } }
    }

    fn finish_transaction(&self, transaction: Transaction<Self::TransactionImplementation>) {
        *self.connection.borrow_mut() = Some(transaction.transaction.connection)
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

        let borrow = self.connection.borrow();

        // TODO SECURITY this is unsafe
        let mut stmt = (*borrow).as_ref().unwrap()
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

        let borrow = self.connection.borrow();
        // TODO SECURITY this is unsafe
        let mut stmt = (*borrow).as_ref().unwrap()
            .prepare(&format!("SELECT * FROM {} where id = {}", T::relation_name(), q.parameters().id))
            .unwrap();

        let mut resultvec: Vec<T> = stmt.query_map(&[], |row| {
            T::from(row)
        }).unwrap().map(Result::unwrap).collect();

        let res = if resultvec.len() > 0 {
            Some(resultvec.remove(0))
        } else {
            None
        };

        future::ready(res)
    }
}