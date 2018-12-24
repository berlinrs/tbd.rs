use std::collections::HashMap;

use tbd_relation::Relation;

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

#[derive(Default)]
pub struct PostFieldSet {
    pub content: ContentField
}

impl AssociatedFieldSet for Post {
    type Set = PostFieldSet;
}

impl FieldSet for PostFieldSet {
    type Model = Post;
    type Marker = Complete;

    fn names() -> Vec<&'static str> {
        let mut vec = Vec::new();
        vec.push(ContentField::name());
        vec
    }
}

impl ModelLifeCycle for Post {
    type PrimaryKey = i64;

    fn created(&mut self, pk: &[u8]) {

    }
}

pub struct KeyedPost(pub Keyed<i64, Post>);

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

pub struct Posts;

impl Relation for Posts {
    type PrimaryKey = i64;
    type Model = Post;
    type Fields = PostFieldSet;
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