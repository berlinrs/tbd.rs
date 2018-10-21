#![feature(async_await, await_macro, futures_api)]

mod schema;
mod models;

use crate::models::post::*;
use crate::models::comment::*;
use crate::schema::*;

use std::cell::RefCell;

use futures::StreamExt;
use futures::future;
use futures::future::FutureObj;
use futures::task::Spawn;

use rusqlite::Connection;

use tbd_sqlite3::Sqlite3Gateway;
use tbd_core::query::*;
use tbd_core::changeset::*;
use tbd_core::repository::{Stores, Repository};

use tbd_keyed::Keyed;


struct BlogRepository {
    gateway: Sqlite3Gateway
}

impl Repository for BlogRepository {
    type Gateway = Sqlite3Gateway;

    fn gateway(&self) -> &Sqlite3Gateway {
        &self.gateway
    }
}


impl Stores<Posts> for BlogRepository {

}

impl Stores<Comments> for BlogRepository {

}


async fn read_from_repos() {
    let conn = Connection::open_in_memory().unwrap();

    conn.execute(
        "CREATE TABLE posts (
                  id              INTEGER PRIMARY KEY,
                  content         TEXT NOT NULL
                  )",
        &[],
    ).unwrap();

    conn.execute(
        "CREATE TABLE comments (
                  id              INTEGER PRIMARY KEY,
                  content         TEXT NOT NULL,
                  post_id         INTEGER)",
        &[],
    ).unwrap();

    let mut changeset = BlogRepository::change().inserts::<Posts>();

    for id in 1..=3 {
        let post = Post { content: format!("Post number {}", id) };

        changeset.push(KeyedPost(Keyed::new(post)));
    }

    let mut changeset = changeset.inserts::<Comments>();

    for id in 1..=9 {
        let post_id = id % 3;
        changeset.push(
            Keyed::new(
                Comment {
                    content: format!("Comment number {} on post {}", id, post_id + 1),
                    post_id: post_id + 1
                }
            )
        )
    }
    
    let gateway = Sqlite3Gateway { connection: RefCell::new(Some(conn)) };
    let repos = BlogRepository { gateway: gateway };

    changeset.commit(&repos);

    let query = select::<Post>().from::<Posts>();

    let e1 = query.execute(&repos).for_each(|item| {
        println!("{:?}", item);
        future::ready(())
    });

    await!(e1);

    let query2 = select::<Comment>().from::<Comments>();

    let e2 = query2.execute(&repos).for_each(|item| {
        println!("{:?}", item);
        future::ready(())
    });

    await!(e2);

    let query3 = select::<Post>().from::<Posts>().find(1);

    println!("{:?}", await!(query3.execute(&repos)));

    let query4 = select::<Post>().from::<Posts>().find(365);

    println!("{:?}", await!(query4.execute(&repos)));
}

fn main() {
    let executor = mini_exec::Executor::new();

    (&executor).spawn_obj(FutureObj::new(Box::new(
        read_from_repos()
    ))).unwrap();

    executor.run();
}