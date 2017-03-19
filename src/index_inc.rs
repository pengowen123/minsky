//! Increment at index operation

use ::*;

pub trait IndexIncrement<N> {
    type Output: Default;
}

pub type IndexInc<L, N> = <L as IndexIncrement<N>>::Output;

impl<L, N> IndexIncrement<N> for L
    where L: RotateLeftBy<N>,
          <L as RotateLeftBy<N>>::Output: IncrementHead,
          <<L as RotateLeftBy<N>>::Output as IncrementHead>::Output: RotateRightBy<N>
{
    type Output = RotRightBy<IncHead<RotLeftBy<L, N>>, N>;
}
