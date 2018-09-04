#![feature(async_await, await_macro, futures_api, pin, arbitrary_self_types, specialization)]

use rusqlite::Connection;

struct Sqlite3Gateway {
    connection: Connection
}