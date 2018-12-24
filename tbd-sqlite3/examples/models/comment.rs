use tbd_model_wrappers::Wrapper;
use tbd_lifecycle::ModelLifeCycle;
use tbd_fieldset::*;

use tbd_keyed::Keyed;   

#[derive(Debug, Clone)]
pub struct Comment {
    // TODO: These pubs must go
    pub content: String,
    pub post_id: i64
}

#[derive(Clone, Default)]
pub struct ContentField(String);


#[derive(Clone, Default)]
pub struct PostId(i64);

impl Field for ContentField {
    type Model = Comment;
    type Type = String;

    fn name() -> &'static str {
        "content"
    }

    fn get(model: &Comment) -> &String {
        &model.content
    }

     fn get_mut(model: &mut Comment) -> &mut String {
        &mut model.content
    }
}

impl Field for PostId {
    type Model = Comment;
    type Type = i64;

    fn name() -> &'static str {
        "post_id"
    }

    fn get(model: &Comment) -> &i64 {
        &model.post_id
    }

     fn get_mut(model: &mut Comment) -> &mut i64 {
        &mut model.post_id
    }
}

pub struct CommentFieldSet {
    pub content: ContentField,
    pub id: PostId
}

impl FieldSet for CommentFieldSet {
    type Model = Comment;
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

pub struct KeyedComment(pub Keyed<i64, Comment>);

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