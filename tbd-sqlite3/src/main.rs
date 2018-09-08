#![feature(async_await, await_macro, futures_api, pin, arbitrary_self_types, specialization)]

use std::future::FutureObj;
use std::task::Spawn;
use std::collections::HashMap;
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

impl<'a> From<&'a rusqlite::Row<'a, 'a>> for Comment {
    fn from(row: &rusqlite::Row) -> Comment {
        Comment {
            id: row.get(0),
            content: row.get(1),
            post_id: row.get(2)
        }
    }
}

#[derive(Debug, Clone)]
struct Comment {
    id: i64,
    content: String,
    post_id: i64
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
    type PrimaryKey = i64;
    type Model = Post;

    fn hydrate(model: &Post) -> HashMap<String, String> {
        let mut h = HashMap::new();
        h.insert("id".to_string(), model.id.to_string());
        h.insert("content".to_string(), format!("{}{}{}", '"', model.content.to_string(), '"'));
        h
    }

    fn name() -> &'static str {
        "posts"
    }
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
    type PrimaryKey = i64;
    type Model = Comment;

    fn hydrate(model: &Comment) -> HashMap<String, String> {
        let mut h = HashMap::new();
        h.insert("id".to_string(), model.id.to_string());
        h.insert("content".to_string(), format!("{}{}{}", '"', model.content.to_string(), '"'));
        h.insert("post_id".to_string(), model.post_id.to_string());
        h
    }

     fn name() -> &'static str {
        "comments"
    }
}


impl RelationName for Comment {
    fn relation_name() -> &'static str {
        "comments"
    }
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

    conn.execute(
        "CREATE TABLE comments (
                  id              INTEGER PRIMARY KEY,
                  content         TEXT NOT NULL,
                  post_id         INTEGER)",
        &[],
    ).unwrap();

    let mut changeset = BlogRepository::change().inserts::<Posts>();

    for id in 1..=3 {
        let post = Post { id: id, content: format!("Post number {}", id) };

        changeset.push(post);
    }

    let mut changeset = changeset.inserts::<Comments>();

    for id in 1..=9 {
        let post_id = id % 3;
        changeset.push(
            Comment {
                id: id,
                content: format!("Comment number {} on post {}", id, post_id + 1),
                post_id: post_id + 1
            }
        )
    }
    
    let gateway = Sqlite3Gateway { connection: conn };
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
}

fn main() {
    let executor = mini_exec::Executor::new();

    (&executor).spawn_obj(FutureObj::new(Box::new(
        read_from_repos()
    ))).unwrap();

    executor.run();
}