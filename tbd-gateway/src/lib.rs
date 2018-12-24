pub mod transaction;

use crate::transaction::{Transaction, TransactionImplementation};

pub trait Gateway {
    type TransactionImplementation: TransactionImplementation;

    fn start_transaction(&self) -> Transaction<Self::TransactionImplementation>;
    fn finish_transaction(&self, transaction: Transaction<Self::TransactionImplementation>);
}

pub trait Autogenerates<T> : Gateway {}
