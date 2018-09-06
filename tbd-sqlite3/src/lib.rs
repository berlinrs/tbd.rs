#![feature(async_await, await_macro, futures_api, pin, arbitrary_self_types, specialization)]

use rusqlite::Connection;
use tbd_core::types::*;
use futures::stream;

pub struct Sqlite3Gateway {
    pub connection: Connection
}

impl Gateway for Sqlite3Gateway {

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

        let mut stmt = self.connection
            .prepare(&format!("SELECT * FROM {}", T::relation_name()))
            .unwrap();

        let resultvec: Vec<T> = stmt.query_map(&[], |row| {
            T::from(row)
        }).unwrap().map(Result::unwrap).collect();

        stream::iter(resultvec.into_iter())
    }
}