use crate::repository::{Repository, Stores};
use tbd_relation::Relation;
use tbd_gateway::Gateway;
use tbd_gateway::transaction::{Transaction, TransactionImplementation};

use std::cell::RefCell;
use std::future::Future;
use futures::future::Ready;

pub trait Changes 
    where Self: Repository + Sized {
    fn change() -> Changeset<Self>;
}

pub trait Changeable<R> where R: Repository,
                              Self: Sized {
    //type Future: Future;

    fn insert<Rel>(self, m: Rel::Wrapper) -> Change<Self, R, Insert<Rel>>
        where R: Stores<Rel>,
              Rel: Relation;

    fn inserts<Rel>(self) -> ChangeList<Self, R, Insert<Rel>, Rel>
        where R: Stores<Rel>,
              Rel: Relation;

    fn build_transaction(&self, repos: &R) -> Transaction<<R::Gateway as Gateway>::TransactionImplementation>;

    fn commit(&self, repos: &R);
}

pub trait Operation {
    fn apply_on_transaction<T>(&self, t: &mut T) where T: TransactionImplementation;
}

impl<R,G> Changes for R
    where R: Repository<Gateway=G> + Sized,
          G: Gateway {
    fn change() -> Changeset<Self> {
        Changeset { relation: std::marker::PhantomData }
    }
}

pub struct Insert<R: Relation> {
    insert: RefCell<R::Wrapper>
}

impl<R> Operation for Insert<R> where R: Relation {
    fn apply_on_transaction<T>(&self, t: &mut T) where T: TransactionImplementation {
        t.insert::<R>(&mut self.insert.borrow_mut());
    }
}

pub struct Changeset<R: Repository> {
    relation: std::marker::PhantomData<R>
}

impl<R> Changeable<R> for Changeset<R> where R: Repository {
    //type Future = Ready<()>;

    fn insert<Rel>(self, m: Rel::Wrapper) -> Change<Self, R, Insert<Rel>>
        where R: Stores<Rel>,
              Rel: Relation {
        Change { after: self, operation: Insert { insert: RefCell::new(m) }, marker: std::marker::PhantomData }
    }

    fn inserts<Rel>(self) -> ChangeList<Self, R, Insert<Rel>, Rel>
        where R: Stores<Rel>,
              Rel: Relation {
        ChangeList { after: self, operations: Vec::new(), marker: std::marker::PhantomData, marker_rel: std::marker::PhantomData }
    }

    #[inline]
    fn build_transaction(&self, repos: &R) -> Transaction<<<R as Repository>::Gateway as Gateway>::TransactionImplementation> {
        repos.gateway().start_transaction()
    }

    #[inline]
    fn commit(&self, repos: &R) {
        //futures::future::ok(())
    }
}

pub struct Change<C, R, O> where C: Changeable<R>,
                                 R: Repository,
                                 O: Operation {
    pub after: C,
    pub operation: O,
    marker: std::marker::PhantomData<R>
}

impl<C, R, O> Changeable<R> for Change<C, R, O> 
    where C: Changeable<R>,
          R: Repository,
          O: Operation {

    fn insert<Rel>(self, m: Rel::Wrapper) -> Change<Self, R, Insert<Rel>>
        where R: Stores<Rel>,
              Rel: Relation {
        Change { after: self, operation: Insert { insert: RefCell::new(m) }, marker: std::marker::PhantomData }
    }

    fn inserts<Rel>(self) -> ChangeList<Self, R, Insert<Rel>, Rel>
        where R: Stores<Rel>,
              Rel: Relation {
        ChangeList { after: self, operations: Vec::new(), marker: std::marker::PhantomData, marker_rel: std::marker::PhantomData }
    }

    #[inline]
    fn build_transaction(&self, repos: &R) -> Transaction<<R::Gateway as Gateway>::TransactionImplementation> {
        let mut transaction = self.after.build_transaction(repos);
        self.operation.apply_on_transaction(&mut transaction.transaction);
        transaction
    }

    #[inline]
    fn commit(&self, repos: &R) {
        let transaction = self.build_transaction(repos);
        repos.gateway().finish_transaction(transaction);
    }
}

pub struct ChangeList<C, R, O, Rel> where C: Changeable<R>,
                                     R: Repository + Stores<Rel>,
                                     Rel: Relation,
                                     O: Operation {
    pub after: C,
    pub operations: Vec<O>,
    marker: std::marker::PhantomData<R>,
    marker_rel: std::marker::PhantomData<Rel>
}

impl<C, R, Rel> ChangeList<C, R, Insert<Rel>, Rel> where C: Changeable<R>,
                                        R: Repository + Stores<Rel>,
                                        Rel: Relation {
    
    pub fn push(&mut self, m: Rel::Wrapper) {
        self.operations.push(Insert { insert: RefCell::new(m) });
    }
}

impl<C, R, O, Rel> Changeable<R> for ChangeList<C, R, O, Rel> 
    where C: Changeable<R>,
          R: Repository + Stores<Rel>,
          O: Operation,
          Rel: Relation {

    fn insert<OtherRel>(self, m: OtherRel::Wrapper) -> Change<Self, R, Insert<OtherRel>>
        where R: Stores<OtherRel>,
              OtherRel: Relation {
        Change { after: self, operation: Insert { insert: RefCell::new(m) }, marker: std::marker::PhantomData }
    }

    fn inserts<OtherRel>(self) -> ChangeList<Self, R, Insert<OtherRel>, OtherRel>
        where R: Stores<OtherRel>,
              OtherRel: Relation {
        ChangeList { after: self, operations: Vec::new(), marker: std::marker::PhantomData, marker_rel: std::marker::PhantomData }
    }

    #[inline]
    fn build_transaction(&self, repos: &R) -> Transaction<<R::Gateway as Gateway>::TransactionImplementation>{
        let mut transaction = self.after.build_transaction(repos);
        for op in self.operations.iter() {
            op.apply_on_transaction::<<R::Gateway as Gateway>::TransactionImplementation>(&mut transaction.transaction);
        }
        transaction
    }

    #[inline]
    fn commit(&self, repos: &R) {
        let transaction = self.build_transaction(repos);
        repos.gateway().finish_transaction(transaction);
    }
}