#![feature(rust_2018_preview, async_await, await_macro, futures_api, pin, arbitrary_self_types)]

extern crate futures;

mod mini_exec;
mod types;
mod query;

use std::future::FutureObj;
use std::task::Executor;
use futures::StreamExt;
use futures::stream;
use futures::future;

use crate::types::*;
use crate::query::*;

#[derive(Debug, Clone)]
struct Post {
    id: u64,
    content: String
}

#[derive(Debug, Clone)]
struct Comment {
    id: u64,
    content: String,
    post_id: u64
}

struct MemoryGateway  {
    posts: Vec<Post>,
    comments: Vec<Comment>
}

impl Gateway for MemoryGateway {}

struct BlogRepository {
    gateway: MemoryGateway
}

impl Repository for BlogRepository {}

struct Posts;

impl Relation<BlogRepository> for Posts {
    type PrimaryKey = u64;
    type Model = Post;
    type Error = ();

    type Stream = stream::Iter<std::vec::IntoIter<Post>>;
    type Future = futures::future::Ready<Option<Post>>;

    fn all(&self, repo: &BlogRepository) -> Self::Stream {
        stream::iter(repo.gateway.posts.clone().into_iter())
    }

    fn one(&self, id: u64, repo: &BlogRepository) -> Self::Future {
        future::ready(repo.gateway.posts.iter().find(|p| p.id == id).cloned())
    }
}

struct Comments;

impl Relation<BlogRepository> for Comments {
    type PrimaryKey = u64;
    type Model = Comment;
    type Error = ();

    type Stream = stream::Iter<std::vec::IntoIter<Comment>>;
    type Future = futures::future::Ready<Option<Comment>>;

    fn all(&self, repo: &BlogRepository) -> Self::Stream {
        stream::iter(repo.gateway.comments.clone().into_iter())
    }

    fn one(&self, id: u64, repo: &BlogRepository) -> Self::Future {
        future::ready(repo.gateway.comments.iter().find(|c| c.id == id).cloned())
    }
}

struct PostComments;

impl HasManyRelationShip<BlogRepository, BlogRepository> for PostComments {
    type Of = Posts;
    type To = Comments;
}

struct CommentPost;

impl BelongsToRelationship<BlogRepository, BlogRepository> for CommentPost {
    type Source = Comments;
    type To = Posts;
}


async fn read_from_repos() {
    let mut posts = vec![];

    for id in 1..=3 {
        posts.push(
            Post {
                id: id,
                content: format!("Post number {}", id)
            }
        )
    }

    let mut comments = vec![];

    for id in 1..=9 {
        let post_id = id % 3;
        comments.push(
            Comment {
                id: id,
                content: format!("Comment number {} on post {}", id, post_id + 1),
                post_id: post_id + 1
            }
        )
    }

    let gateway = MemoryGateway { posts, comments };
    let repos = BlogRepository { gateway };

    let query = select::<Post>().from(&Posts);

    let q1 = query.execute(&repos).for_each(|item| {
        println!("{:?}", item);
        future::ready(())
    });

    await!(q1);

    let f1 = Posts.all(&repos).for_each(|item|{
        println!("{:?}", item);
        future::ready(())
    });

    await!(f1);

    let f2 = Comments.all(&repos).for_each(|item|{
        println!("{:?}", item);
        future::ready(())
    });

    await!(f2);

    let model = await!(Comments.one(2, &repos));

    println!("{:?}", model);
}

fn main() {
    let executor = mini_exec::Executor::new();

    (&executor).spawn_obj(FutureObj::new(Box::new(
        read_from_repos()
    ))).unwrap();

    executor.run();
}