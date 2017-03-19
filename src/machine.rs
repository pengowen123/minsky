//! Machine state

use std::marker::PhantomData;

use ::*;

/// The state of the Minsky machine, storing the instructions (`I`), registers (`R`), and index of
/// the current instruction (`N`).
#[derive(Default, Debug)]
pub struct Machine<I, R, N, D>(PhantomData<(I, R, N, D)>);

pub trait Cycle {
    type Next;
}

pub type RunCycle<M> = <M as Cycle>::Next;

#[cfg_attr(rustfmt, rustfmt_skip)]
impl<I, R, N, D> Cycle for Machine<I, R, N, D>
    where I: RotateLeftBy<N>,
          <I as RotateLeftBy<N>>::Output: GetFirstTrait,
          Instruction<<<I as RotateLeftBy<N>>::Output as GetFirstTrait>::Output, R, I, D>:
              ExecuteInstruction<R, I>,
          ExecInstruction<Index<I, N>, R, I, D>: Cycle
{
    type Next = RunCycle<ExecInstruction<Index<I, N>, R, I, D>>;
}

// Terminate
impl<R, D> Cycle for Output<R, D> {
    type Next = Output<R, D>;
}
