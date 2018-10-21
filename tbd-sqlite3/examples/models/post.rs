use tbd_core::model_wrappers::Wrapper; 
use tbd_core::lifecycle::ModelLifeCycle;

use tbd_keyed::Keyed;  

#[derive(Debug, Clone)]
pub struct Post {
    // TODO: These pubs must go
    pub content: String
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

pub struct KeyedPost(pub Keyed<i64, Post>);

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
