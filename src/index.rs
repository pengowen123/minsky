//! Index operation

use ::*;

pub trait IndexTrait<I> {
    type Output: Default;
}

pub type Index<T, I> = <T as IndexTrait<I>>::Output;

impl<L, N> IndexTrait<N> for L
    where L: RotateLeftBy<N>,
          <L as RotateLeftBy<N>>::Output: GetFirstTrait
{
    type Output = GetFirst<<L as RotateLeftBy<N>>::Output>;
}
