//! RotateRight operation

use ::*;

pub trait RotateRight {
    type Output: Default;
}

pub type RotRight<L> = <L as RotateRight>::Output;

impl<L> RotateRight for L
    where L: RemoveLastTrait + GetLastTrait
{
    type Output = Cons<GetLast<L>, RemoveLast<L>>;
}
