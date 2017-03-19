//! Get last element operation

use ::*;

pub trait GetLastTrait {
    type Output: Default;
}

pub type GetLast<L> = <L as GetLastTrait>::Output;

impl<H> GetLastTrait for Cons<H, Nil>
    where H: Default
{
    type Output = H;
}

impl<H, T> GetLastTrait for Cons<H, T>
    where T: GetLastTrait
{
    type Output = T::Output;
}
