//! Increment first element operation

use ::*;

pub trait IncrementHead {
    type Output;
}

pub type IncHead<L> = <L as IncrementHead>::Output;

impl<H, T> IncrementHead for Cons<H, T> {
    type Output = Cons<Succ<H>, T>;
}
