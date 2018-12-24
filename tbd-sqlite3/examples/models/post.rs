use tbd_model_wrappers::Wrapper;
use tbd_lifecycle::ModelLifeCycle;
use tbd_fieldset::*;

use tbd_keyed::Keyed;  

#[derive(Debug, Clone)]
pub struct Post {
    // TODO: These pubs must go
    pub content: String
}

#[derive(Clone, Default)]
pub struct ContentField(String);

impl Field for ContentField {
    type Model = Post;
    type Type = String;

    fn name() -> &'static str {
        "content"
    }

    fn get(model: &Post) -> &String {
        &model.content
    }

     fn get_mut(model: &mut Post) -> &mut String {
        &mut model.content
    }
}

pub struct PostFieldSet {
    pub content: ContentField
}

impl FieldSet for PostFieldSet {
    type Model = Post;
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
