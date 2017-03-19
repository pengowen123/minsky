//! Get first element operation

use ::*;

pub trait GetFirstTrait {
    type Output: Default;
}

pub type GetFirst<L> = <L as GetFirstTrait>::Output;

impl GetFirstTrait for Nil {
    type Output = Nil;
}

impl<H, T> GetFirstTrait for Cons<H, T>
    where H: Default
{
    type Output = H;
}
