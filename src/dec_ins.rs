//! Decrement a register unless it is zero, in which case go to the alternate instruction

use ::*;

pub trait DecrementUnlessZero {
    type Output: Default;
}

pub type DecUnlessZero<V, Reg, R, Ins, P, P2, D> = <(V, Reg, R, Ins, P, P2, D) as DecrementUnlessZero>::Output;

impl<N, Reg, R, Ins, P, P2, D> DecrementUnlessZero for (Succ<N>, Reg, R, Ins, P, P2, D)
    where Reg: RotateLeftBy<R>,
          <Reg as RotateLeftBy<R>>::Output: DecrementHead,
          <<Reg as RotateLeftBy<R>>::Output as DecrementHead>::Output: RotateRightBy<R>,
          Machine<Ins, IndexDec<Reg, R>, P, D>: Default,
          D: Append<Decrement<R, Nil, P2>>,
          Ins: Default,
          P2: Default
{
    type Output = Machine<Ins, IndexDec<Reg, R>, P2, D::Output>;
}

impl<Reg, R, Ins, P, P2, D> DecrementUnlessZero for (Zero, Reg, R, Ins, P, P2, D)
    where Machine<Ins, Reg, P2, D::Output>: Default,
          Ins: Default,
          Reg: Default,
          P: Default,
          D: Append<Decrement<R, P, Nil>>
{
    type Output = Machine<Ins, Reg, P, D::Output>;
}
