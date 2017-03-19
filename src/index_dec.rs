//! Decrement at index operation

use ::*;

pub trait IndexDecrement<N> {
    type Output: Default;
}

pub type IndexDec<L, N> = <L as IndexDecrement<N>>::Output;

impl<L, N> IndexDecrement<N> for L
    where L: RotateLeftBy<N>,
          <L as RotateLeftBy<N>>::Output: DecrementHead,
          <<L as RotateLeftBy<N>>::Output as DecrementHead>::Output: RotateRightBy<N>
{
    type Output = RotRightBy<DecHead<RotLeftBy<L, N>>, N>;
}
