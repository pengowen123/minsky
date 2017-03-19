//! Remove last operation

use ::*;

pub trait RemoveLastTrait {
    type Output: Default;
}

pub type RemoveLast<L> = <L as RemoveLastTrait>::Output;

impl<H> RemoveLastTrait for Cons<H, Nil> {
    type Output = Nil;
}

impl<H, T> RemoveLastTrait for Cons<H, T>
    where H: Default,
          T: RemoveLastTrait
{
    type Output = Cons<H, T::Output>;
}
