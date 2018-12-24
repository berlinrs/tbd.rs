use crate::models::post::*;
use crate::models::comment::*;

use std::collections::HashMap;

use tbd_core::relationships::*;
use tbd_relation::Relation;

use tbd_keyed::Keyed;

use tbd_sqlite3::RelationName;

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


impl RelationName for Post {
    fn relation_name() -> &'static str {
        "posts"
    }
}

pub struct Comments;

impl Relation for Comments {
    type PrimaryKey = i64;
    type Model = Comment;
    type Fields = CommentFieldSet;
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