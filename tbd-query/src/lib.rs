pub mod compile;

use tbd_relation::Relation;
use tbd_fieldset::*;

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

pub struct FindParameters<PK> {
    pub id: PK
}

pub struct Find<PK, Q> {
    params: FindParameters<PK>,
    query: Limit<Q>,
}

impl<PK, Q> Find<PK, Q> {
    fn new(id: PK, query: Q) -> Self {
        Find { params: FindParameters { id: id }, query: Limit { max: 1, inner: query }}
    }
}

impl<PK, Q> Query for Find<PK, Q> where Q: Query {
    type ReturnType = Q::ReturnType;
    type QueryMarker = One;
    type Parameters = FindParameters<PK>;

    fn parameters(&self) -> &FindParameters<PK> {
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
        where R: Relation<Model = F::Model> {
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
    where F:FieldSet,
          R: Relation {

    pub fn limit(self, max: usize) -> Limit<Self> {
        Limit { max: max, inner: self }
    }

    pub fn find(self, id: R::PrimaryKey) -> Find<R::PrimaryKey, Self> {
        Find::new(id, self)
    }
}
