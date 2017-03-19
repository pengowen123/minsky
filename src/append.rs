//! Append operation

use ::*;

pub trait Append<Elem> {
    type Output: Default;
}

impl<E> Append<E> for Nil
    where E: Default
{
    type Output = Cons<E, Nil>;
}

impl<H, T, E> Append<E> for Cons<H, T>
    where H: Default,
          T: Append<E>,
          E: Default
{
    type Output = Cons<H, T::Output>;
}
