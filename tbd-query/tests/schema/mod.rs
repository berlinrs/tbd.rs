use std::collections::HashMap;

use tbd_relation::Relation;
use tbd_relation::fieldset::*;

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
pub struct IdField;

impl Field for IdField {
    type Type = i64;
}

impl PrimaryField for IdField {}


#[derive(Clone, Default)]
pub struct ContentField;

impl Field for ContentField {
    type Type = String;
}

#[derive(Default)]
pub struct PostFieldSet {
    pub id: IdField,
    pub content: ContentField
}

impl AssociatedFieldSet for Post {
    type Set = PostFieldSet;
}

impl FieldSet for PostFieldSet {
    type Marker = Complete;
}

pub struct Posts;

impl Relation for Posts {
    type PrimaryKey = i64;
    type PrimaryField = IdField;
    type Fields = PostFieldSet;

    fn name() -> &'static str {
        "posts"
    }
}

impl RelationField for IdField {
    type Relation = Posts;

    fn name() -> &'static str {
        "id"
    }
}

impl RelationField for ContentField {
    type Relation = Posts;

    fn name() -> &'static str {
        "content"
    }
}

impl RelationFieldSet for PostFieldSet {
    type Relation = Posts;

    fn names() -> Vec<&'static str> {
        let mut vec = Vec::new();
        vec.push(IdField::name());
        vec.push(ContentField::name());
        vec
    }
}
