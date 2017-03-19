//! Printing of final register values

use std::marker::PhantomData;
use std::fmt;

#[derive(Default)]
pub struct Output<R, D>(PhantomData<(R, D)>);

impl<R, D> fmt::Debug for Output<R, D>
    where R: Default + fmt::Debug,
          D: Default + fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Final register state: {:?}\nLog: {:?}",
               R::default(),
               D::default())
    }
}
