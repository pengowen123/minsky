//! Rotate left operation

use ::*;

pub trait RotateLeft {
    type Output: Default;
}

pub type RotLeft<T> = <T as RotateLeft>::Output;

impl RotateLeft for Nil {
    type Output = Nil;
}

impl<H, T> RotateLeft for Cons<H, T>
    where T: ConsCell + Append<H>
{
    type Output = T::Output;
}
