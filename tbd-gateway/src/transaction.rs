use tbd_relation::Relation;

pub trait TransactionImplementation where Self: Sized {
    fn insert<R>(&mut self, m: &mut R::Fields) where R: Relation;
}

pub struct Transaction<T> 
    where T: TransactionImplementation {
    pub transaction: T
}