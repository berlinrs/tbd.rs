pub mod compile;

use tbd_relation::Relation;
use tbd_relation::fieldset::*;
use tbd_fieldset::*;

use std::fmt::Display;
pub trait QueryMarker {}

pub struct One;
pub struct All;
pub struct Incomplete;

impl QueryMarker for One {}
impl QueryMarker for All {}
impl QueryMarker for Incomplete {}

pub trait Query {
    type ReturnType: FieldSet;
    type QueryMarker: QueryMarker;
    type Parameters;

    fn parameters(&self) -> &Self::Parameters;
}

impl<Q> Query for &Q where Q: Query {
    type ReturnType = Q::ReturnType;
    type QueryMarker = Q::QueryMarker;
    type Parameters = Q::Parameters;

    fn parameters(&self) -> &Self::Parameters {
        (*self).parameters()
    }
}

pub struct Limit<Q> {
    max: usize,
    inner: Q,
}


impl<Q> Query for Limit<Q> where Q: Query {
    type ReturnType = Q::ReturnType;
    type QueryMarker = One;
    type Parameters = ();

    fn parameters(&self) -> &() {
        &()
    }
}

impl<Q> Limit<Q> where Q: Query {
    pub fn max(&self) -> usize {
        self.max
    }

    pub fn inner(&self) -> &Q {
        &self.inner
    }
}

// TODO: Display should be something else, ToParam, for example
pub struct FindParameters<F> where F: Field, F::Type: Display {
    pub parameter: F::Type
}

pub struct Find<F, Q> where F: Field, F::Type: Display {
    params: FindParameters<F>,
    query: Q,
}

impl<F, Q> Find<F, Q> where F: Field, F::Type: Display {
    fn new(parameter: F::Type, query: Q) -> Self {
        Find { params: FindParameters { parameter: parameter }, query: query}
    }

    pub fn parameter(&self) -> &F::Type {
        &self.params.parameter
    }

    pub fn inner(&self) -> &Q {
        &self.query
    }
}

impl<F, Q> Query for Find<F, Q> where Q: Query, F: Field, F::Type: Display {
    type ReturnType = Q::ReturnType;
    type QueryMarker = One;
    type Parameters = FindParameters<F>;

    fn parameters(&self) -> &FindParameters<F> {
        &self.params
    }
}

pub struct Select<F: FieldSet> {
    f: std::marker::PhantomData<F>,
}

impl<F> Query for Select<F> where F: FieldSet {
    type ReturnType = F;
    type QueryMarker = Incomplete;
    type Parameters = ();

    fn parameters(&self) -> &() {
        &()
    }
}

pub fn select<F>() -> Select<F::Set> where F: AssociatedFieldSet {
    Select { f: std::marker::PhantomData }
}

impl<F> Select<F> where F: FieldSet {
    pub fn from<R>(self) -> SelectFrom<F, R>
        where F: RelationFieldSet<Relation = R>,
              R: Relation {
            SelectFrom { fieldset: std::marker::PhantomData, relation: std::marker::PhantomData }
    }
}

pub struct SelectFrom<F, R>
    where F: FieldSet,
          R: Relation {
    fieldset: std::marker::PhantomData<F>,
    relation: std::marker::PhantomData<R>,
}

impl<F, R> Query for SelectFrom<F, R>
    where F: FieldSet,
          R: Relation {
    type ReturnType = F;
    type QueryMarker = All;

    type Parameters = ();

    fn parameters(&self) -> &() {
        &()
    }
}

impl<F, R> SelectFrom<F, R> 
    where F: FieldSet,
          R: Relation,
          R::PrimaryKey: Display {

    pub fn limit(self, max: usize) -> Limit<Self> {
        Limit { max: max, inner: self }
    }

    pub fn find(self, id: R::PrimaryKey) -> Find<R::PrimaryField, Self> {
        Find::new(id, self)
    }
}
