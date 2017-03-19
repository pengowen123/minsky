//! RotateRightBy operation (equivalent to N RotateRight operations)

use ::*;

pub trait RotateRightBy<N>: RotateRight {
    type Output: Default;
}

pub type RotRightBy<L, N> = <L as RotateRightBy<N>>::Output;

impl<H, T> RotateRightBy<Zero> for Cons<H, T>
    where H: Default,
          T: Default + ConsCell,
          Cons<H, T>: RotateRight
{
    type Output = Cons<H, T>;
}

impl<H, T, N> RotateRightBy<Succ<N>> for Cons<H, T>
    where H: Default,
          T: Default + ConsCell,
          Cons<H, T>: RotateRight,
          <Cons<H, T> as RotateRight>::Output: RotateRightBy<N>
{
    type Output = RotRightBy<RotRight<Cons<H, T>>, N>;
}
