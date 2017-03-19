//! Decrement first element operation

use ::*;

pub trait DecrementHead {
    type Output;
}

pub type DecHead<L> = <L as DecrementHead>::Output;

impl<H, T> DecrementHead for Cons<Succ<H>, T> {
    type Output = Cons<H, T>;
}
