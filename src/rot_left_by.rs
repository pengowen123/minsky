//! RotateLeftBy operation (equivalent to N RotateLeft operations)

use ::*;

pub trait RotateLeftBy<N>: RotateLeft {
    type Output: Default;
}

pub type RotLeftBy<T, N> = <T as RotateLeftBy<N>>::Output;

impl<N> RotateLeftBy<N> for Nil {
    type Output = Nil;
}

impl<H, T> RotateLeftBy<Zero> for Cons<H, T>
    where H: Default,
          T: Default + ConsCell,
          T: Append<H>
{
    type Output = Cons<H, T>;
}

impl<H, T, N> RotateLeftBy<Succ<N>> for Cons<H, T>
    where H: Default,
          T: Default + ConsCell,
          T: Append<H>,
          T::Output: RotateLeftBy<N>
{
    type Output = RotLeftBy<RotLeft<Cons<H, T>>, N>;
}
