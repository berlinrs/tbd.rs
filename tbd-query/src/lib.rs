use tbd_relation::Relation;

pub trait QueryMarker {}

pub struct One;
pub struct All;
pub struct Incomplete;

impl QueryMarker for One {}
impl QueryMarker for All {}
impl QueryMarker for Incomplete {}

pub trait Query {
    type ReturnType;
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

pub struct Select<M> {
    m: std::marker::PhantomData<M>,
}

impl<M> Query for Select<M> {
    type ReturnType = M;
    type QueryMarker = Incomplete;
    type Parameters = ();

    fn parameters(&self) -> &() {
        &()
    }
}

pub fn select<M>() -> Select<M> {
    Select { m: std::marker::PhantomData }
}

impl<M> Select<M> {
    pub fn from<R>(self) -> SelectFrom<R>
        where R: Relation<Model = M> {
            SelectFrom { relation: std::marker::PhantomData }
    }
}

pub struct SelectFrom<R>
    where R: Relation {
    relation: std::marker::PhantomData<R>,
}

impl<R> Query for SelectFrom<R>
    where R: Relation {
    type ReturnType = R::Model;
    type QueryMarker = All;

    type Parameters = ();

    fn parameters(&self) -> &() {
        &()
    }
}

impl<R> SelectFrom<R> 
    where R: Relation {

    pub fn limit(self, max: usize) -> Limit<Self> {
        Limit { max: max, inner: self }
    }

    pub fn find(self, id: R::PrimaryKey) -> Find<R::PrimaryKey, Self> {
        Find::new(id, self)
    }
}
