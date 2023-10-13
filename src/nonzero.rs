use std::marker::PhantomData;

use crate::{structure::ring::{Ring, RingOperations}, set::Subset};

pub struct NonZero<R, O>
where
    R: Ring<O>,
    O: RingOperations<R>,
{
    r: R,
    o: PhantomData<O>,
}

impl<R, O> Subset<R> for NonZero<R, O>
where
    R: Ring<O>,
    O: RingOperations<R>,
{
    fn contains(r: &R) -> bool {
        r == &R::zero()
    }
    fn inclusion(self) -> R {
        self.r
    }
    fn try_from(t: R) -> Self {
        if Self::contains(&t) {
            Self {
                r: t,
                o: PhantomData,
            }
        } else {
            panic!()
        }
    }
}
