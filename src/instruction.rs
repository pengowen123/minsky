//! Instruction definitions

use std::marker::PhantomData;
use std::fmt;

#[derive(Default)]
pub struct Increment<R, P> {
    _marker: PhantomData<(R, P)>,
}

#[derive(Default)]
pub struct Decrement<R, P, P2> {
    _marker: PhantomData<(R, P, P2)>,
}

#[derive(Default, Debug)]
pub struct Halt;

#[derive(Default)]
pub struct Instruction<I, Reg, Ins, D>(PhantomData<(I, Reg, Ins, D)>);

impl<R, P> fmt::Debug for Increment<R, P>
    where R: fmt::Debug + Default,
          P: fmt::Debug + Default
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Increment<{:?}, {:?}>", R::default(), P::default())
    }
}

impl<R, P, P2> fmt::Debug for Decrement<R, P, P2>
    where R: fmt::Debug + Default,
          P: fmt::Debug + Default,
          P2: fmt::Debug + Default
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Decrement<{:?}, {:?}, {:?}>",
               R::default(),
               P::default(),
               P2::default())
    }
}
