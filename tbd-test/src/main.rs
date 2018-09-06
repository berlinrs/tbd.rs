#![feature(async_await, await_macro, futures_api, pin, arbitrary_self_types, specialization)]

use std::future::FutureObj;
use std::task::Spawn;
use futures::StreamExt;
use futures::stream;
use futures::future;

use tbd_core::types::*;
use tbd_core::query::*;
use tbd_core::mini_exec;

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


impl ExecuteAll<Post> for MemoryGateway {
    type Error = ();
    type Stream = stream::Iter<std::vec::IntoIter<Post>>;

    fn execute_query<Q>(&self, q: &Q) -> Self::Stream
        where Q: Query<QueryMarker=All, ReturnType=Post> {
        stream::iter(self.posts.clone().into_iter())
    }
}

impl ExecuteAll<Comment> for MemoryGateway {
    type Error = ();
    type Stream = stream::Iter<std::vec::IntoIter<Comment>>;

    fn execute_query<Q>(&self, q: &Q) -> Self::Stream
        where Q: Query<QueryMarker=All, ReturnType=Comment> {
        stream::iter(self.comments.clone().into_iter())
    }
}

impl ExecuteOne<Post, FindParameters<u64>> for MemoryGateway {
    type Error = ();
    type Future = futures::future::Ready<Option<Post>>;

    fn execute_query<Q>(&self, q: &Q) -> Self::Future
        where Q: Query<QueryMarker=One, ReturnType=Post, Parameters=FindParameters<u64>> {
        future::ready(self.posts.iter().find(|p| p.id == q.parameters().id).cloned())
    }
}


struct BlogRepository {
    gateway: MemoryGateway
}

impl Repository for BlogRepository {
    type Gateway = MemoryGateway;

    fn gateway(&self) -> &MemoryGateway {
        &self.gateway
    }
}

struct Posts;

impl Relation for Posts {
    type PrimaryKey = u64;
    type Model = Post;
}

impl Stores<Posts> for BlogRepository {
    // type Error = ();

    // type Stream = stream::Iter<std::vec::IntoIter<Post>>;
    // type Future = futures::future::Ready<Option<Post>>;

    // fn all(&self) -> Self::Stream {
    //     stream::iter(self.gateway.posts.clone().into_iter())
    // }

    // fn one(&self, id: u64) -> Self::Future {
    //     future::ready(self.gateway.posts.iter().find(|p| p.id == id).cloned())
    // }
}

struct Comments;

impl Relation for Comments {
    type PrimaryKey = u64;
    type Model = Comment;
}

impl Stores<Comments> for BlogRepository {
//     type Error = ();

//     type Stream = stream::Iter<std::vec::IntoIter<Comment>>;
//     type Future = futures::future::Ready<Option<Comment>>;

//     fn all(&self) -> Self::Stream {
//         stream::iter(self.gateway.comments.clone().into_iter())
//     }

//     fn one(&self, id: u64) -> Self::Future {
//         future::ready(self.gateway.comments.iter().find(|c| c.id == id).cloned())
//     }
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

    let query3 = select::<Post>().from::<Posts>().find(2);

    let e3 = query3.execute(&repos);

    println!("{:?}", await!(e3));

    // let f1 = <BlogRepository as Stores<Posts>>::all(&repos).for_each(|item|{
    //     println!("{:?}", item);
    //     future::ready(())
    // });

    // await!(f1);

    // let f2 = <BlogRepository as Stores<Comments>>::all(&repos).for_each(|item|{
    //     println!("{:?}", item);
    //     future::ready(())
    // });

    // await!(f2);

    // let model = await!(<BlogRepository as Stores<Comments>>::one(&repos, 2));

    // println!("{:?}", model);
}

fn main() {
    let executor = mini_exec::Executor::new();

    (&executor).spawn_obj(FutureObj::new(Box::new(
        read_from_repos()
    ))).unwrap();

    executor.run();
}