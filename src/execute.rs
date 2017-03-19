//! Execution and implementation of instructions

use ::*;

pub trait ExecuteInstruction<Reg, Ins> {
    type Output: Default;
}

pub type ExecInstruction<I, Reg, Ins, D> = <Instruction<I, Reg, Ins, D> as ExecuteInstruction<Reg, Ins>>::Output;

impl<R, P, Reg, Ins, D> ExecuteInstruction<Reg, Ins> for Instruction<Increment<R, P>, Reg, Ins, D>
    where Reg: RotateLeftBy<R>,
          <Reg as RotateLeftBy<R>>::Output: IncrementHead,
          <<Reg as RotateLeftBy<R>>::Output as IncrementHead>::Output: RotateRightBy<R>,
          Machine<Ins, IndexInc<Reg, R>, P, D::Output>: Default,
          D: Append<Increment<R, P>>,
{
    type Output = Machine<Ins, IndexInc<Reg, R>, P, D::Output>;
}

impl<R, P, P2, Reg, Ins, D> ExecuteInstruction<Reg, Ins>
    for Instruction<Decrement<R, P, P2>, Reg, Ins, D>
    where Reg: RotateLeftBy<R>,
          <Reg as RotateLeftBy<R>>::Output: GetFirstTrait,
          (Index<Reg, R>, Reg, R, Ins, P, P2, D): DecrementUnlessZero
{
    type Output = DecUnlessZero<Index<Reg, R>, Reg, R, Ins, P, P2, D>;
}

impl<Reg, Ins, D> ExecuteInstruction<Reg, Ins> for Instruction<Halt, Reg, Ins, D>
    where Reg: Default,
          D: Append<Halt> + Default
{
    type Output = Output<Reg, D::Output>;
}
