#![feature(async_await, await_macro, futures_api, pin, arbitrary_self_types)]

use std::collections::HashMap;
use std::cell::RefCell;

use futures::StreamExt;
use futures::stream;
use futures::future;
use futures::future::FutureObj;
use futures::task::Spawn;

use rusqlite::Connection;

use tbd_sqlite3::Sqlite3Gateway;
use tbd_sqlite3::RelationName;
use tbd_core::query::*;
use tbd_core::types::*;
use tbd_core::changeset::*;
use tbd_core::model_wrappers::*;

use tbd_keyed::Keyed;

#[derive(Debug, Clone)]
struct Post {
    content: String
}

impl ModelLifeCycle for Post {
    type PrimaryKey = i64;

    fn created(&mut self, pk: &[u8]) {

    }
}

impl<'a> From<&'a rusqlite::Row<'a, 'a>> for Post {
    fn from(row: &rusqlite::Row) -> Post {
        Post {
            content: row.get(1)
        }
    }
}

struct KeyedPost(Keyed<i64, Post>);

impl<'a> From<&'a rusqlite::Row<'a, 'a>> for KeyedPost {
    fn from(row: &rusqlite::Row) -> KeyedPost {
        KeyedPost(Keyed::with_key(
            row.get(0),
            row.into()
        ))
    }
}

impl ModelLifeCycle for KeyedPost {
    type PrimaryKey = i64;

    fn created(&mut self, pk: &[u8]) {
        self.0.created(pk)
    }
}

impl Wrapper for Post {
    type Wrapping = Post;
    type Returning = Post;

    fn wrap(m: Post) -> Post {
        m
    }
}

impl Wrapper for KeyedPost {
    type Wrapping = Post;
    type Returning = KeyedPost;

    fn wrap(m: Post) -> KeyedPost {
        KeyedPost(
            Keyed::new(m)
        )
    }
}


#[derive(Debug, Clone)]
struct Comment {
    content: String,
    post_id: i64
}

impl ModelLifeCycle for Comment {
    type PrimaryKey = i64;

    fn created(&mut self, pk: &[u8]) {

    }
}

impl<'a> From<&'a rusqlite::Row<'a, 'a>> for Comment {
    fn from(row: &rusqlite::Row) -> Comment {
        Comment {
            content: row.get(1),
            post_id: row.get(2)
        }
    }
}

struct KeyedComment(Keyed<i64, Comment>);

impl<'a> From<&'a rusqlite::Row<'a, 'a>> for KeyedComment {
    fn from(row: &rusqlite::Row) -> KeyedComment {
        KeyedComment(Keyed::with_key(
            row.get(0),
            row.into()
        ))
    }
}


impl ModelLifeCycle for KeyedComment {
    type PrimaryKey = i64;

    fn created(&mut self, pk: &[u8]) {
        self.0.created(pk)
    }
}

impl Wrapper for Comment {
    type Wrapping = Comment;
    type Returning = Comment;

    fn wrap(m: Comment) -> Comment {
        m
    }
}

impl Wrapper for KeyedComment {
    type Wrapping = Comment;
    type Returning = KeyedComment;

    fn wrap(m: Comment) -> KeyedComment {
        KeyedComment(
            Keyed::new(m)
        )
    }
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
    type Wrapper = KeyedPost;

    fn hydrate(model: &KeyedPost) -> HashMap<String, String> {
        let model = &model.0;
        let mut h = HashMap::new();
        if let Some(id) = model.pk {
            h.insert("id".to_string(), id.to_string());
        }
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
    type Wrapper = Keyed<Self::PrimaryKey, Comment>;

    fn hydrate(model: &Keyed<Self::PrimaryKey, Comment>) -> HashMap<String, String> {
        let mut h = HashMap::new();
        if let Some(id) = model.pk {
            h.insert("id".to_string(), id.to_string());
        }
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