//! Peano numbers

use std::marker::PhantomData;
use std::fmt;

#[derive(Default)]
pub struct Zero;
#[derive(Default)]
pub struct Succ<T>(PhantomData<T>);

pub trait Peano {}

impl Peano for Zero {}
impl<N> Peano for Succ<N> where N: Peano {}

pub trait ToInt {
    fn to_i32() -> i32;
}

impl ToInt for Zero {
    fn to_i32() -> i32 {
        0
    }
}

impl<N> ToInt for Succ<N>
    where N: ToInt
{
    fn to_i32() -> i32 {
        1 + N::to_i32()
    }
}

impl fmt::Debug for Zero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0")
    }
}

impl<N> fmt::Debug for Succ<N>
    where N: ToInt
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", 1 + N::to_i32())
    }
}
