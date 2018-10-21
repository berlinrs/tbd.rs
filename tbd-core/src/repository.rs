use crate::gateway::Gateway;

pub trait Repository {
    type Gateway: Gateway;

    fn gateway(&self) -> &Self::Gateway;
}

pub trait Stores<R> : Repository {

}

impl<A, B, T> Stores<(A, B)> for T where T: Stores<A> + Stores<B> {} 
