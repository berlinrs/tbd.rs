use tbd_core::model_wrappers::Wrapper; 
use tbd_core::lifecycle::ModelLifeCycle;

use tbd_keyed::Keyed;   

#[derive(Debug, Clone)]
pub struct Comment {
    pub content: String,
    pub post_id: i64
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