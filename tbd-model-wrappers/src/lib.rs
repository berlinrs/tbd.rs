pub trait Wrapper {
    type Wrapping;
    type Returning;

    fn wrap(m: Self::Wrapping) -> Self::Returning;
}

// pub struct Transparent<T> {
//     phantom: std::marker::PhantomData<T>
// }

// impl<T> Wrapper for Transparent<T> {
//     type Wrapping = T;
//     type Returning = T;

//     fn wrap(m: Self::Wrapping) -> Self::Returning {
//         m
//     }
// }