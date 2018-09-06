#![feature(async_await, await_macro, futures_api, pin, arbitrary_self_types, specialization)]

use std::future::FutureObj;
use std::task::Spawn;
use futures::StreamExt;
use futures::stream;
use futures::future;

use rusqlite::Connection;

use tbd_sqlite3::Sqlite3Gateway;
use tbd_sqlite3::RelationName;
use tbd_core::query::*;
use tbd_core::types::*;
use tbd_core::changeset::*;
use tbd_core::mini_exec;

#[derive(Debug, Clone)]
struct Post {
    id: i64,
    content: String
}

impl<'a> From<&'a rusqlite::Row<'a, 'a>> for Post {
    fn from(row: &rusqlite::Row) -> Post {
        Post {
            id: row.get(0),
            content: row.get(1)
        }
    }
}


#[derive(Debug, Clone)]
struct Comment {
    id: u64,
    content: String,
    post_id: u64
}

struct BlogRepository {
    gateway: Sqlite3Gateway
}

impl Repository for BlogRepository {
    type Gateway = Sqlite3Gateway;

    fn gateway(&self) -> &Sqlite3Gateway {
        &self.gateway
    }
}

struct Posts;

impl Relation for Posts {
    type PrimaryKey = u64;
    type Model = Post;
}


impl RelationName for Post {
    fn relation_name() -> &'static str {
        "posts"
    }
}

impl Stores<Posts> for BlogRepository {

}

struct Comments;

impl Relation for Comments {
    type PrimaryKey = u64;
    type Model = Comment;
}

impl Stores<Comments> for BlogRepository {

}

struct PostComments;

impl HasManyRelationShip for PostComments {
    type Of = Posts;
    type To = Comments;
}

struct CommentPost;

impl BelongsToRelationship for CommentPost {
    type Source = Comments;
    type To = Posts;
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

    for id in 1..=3 {
        conn.execute(
            "INSERT INTO posts (id, content)
                  VALUES (?1, ?2)",
            &[&id, &format!("Post number {}", id)],
        ).unwrap();
    }
    
    let gateway = Sqlite3Gateway { connection: conn };
    let repos = BlogRepository { gateway: gateway };

    let post = Post { id: 4, content: "Post number 4".into() };
    let changeset = BlogRepository::change()
                                  .insert::<Posts>(post);

    changeset.commit(&repos);

    let query = select::<Post>().from::<Posts>();

    let e1 = query.execute(&repos).for_each(|item| {
        println!("{:?}", item);
        future::ready(())
    });

    await!(e1);
}

fn main() {
    let executor = mini_exec::Executor::new();

    (&executor).spawn_obj(FutureObj::new(Box::new(
        read_from_repos()
    ))).unwrap();

    executor.run();
}