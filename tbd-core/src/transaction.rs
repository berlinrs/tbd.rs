use crate::relation::Relation;

pub trait TransactionImplementation where Self: Sized {
    fn insert<R>(&mut self, m: &mut R::Wrapper) where R: Relation;
}

pub struct Transaction<T> 
    where T: TransactionImplementation {
    pub transaction: T
}