//! Cons list

use ::*;

#[derive(Default)]
pub struct Nil;

#[derive(Default)]
pub struct Cons<H, T> {
    _marker: PhantomData<(H, T)>,
}

pub trait ConsCell {}

impl ConsCell for Nil {}

impl<H, T> ConsCell for Cons<H, T> where T: ConsCell {}

impl fmt::Debug for Nil {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "()")
    }
}

impl<H, T> fmt::Debug for Cons<H, T>
    where H: fmt::Debug + Default,
          T: fmt::Debug + Default
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        H::default().fmt(f)?;
        write!(f, ", ")?;
        T::default().fmt(f)?;
        write!(f, ")")
    }
}

#[macro_export]
macro_rules! list {
    ($head:ty) => {
        Cons<$head, $crate::cons::Nil>
    };
    ($head:ty, $($tail:ty),*) => {
            Cons<$head, list!($($tail),*)>
    };
}
