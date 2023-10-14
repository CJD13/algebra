use std::marker::PhantomData;

use crate::{structure::ring::{Ring, RingOperations}, set::{Subset, Set}};
pub struct NonZero<R:Ring<O>, O:RingOperations<R>>
{
    r: R,
    o: PhantomData<O>,

}
impl<R, O> PartialEq for NonZero<R, O> where
R: Ring<O>,
O: RingOperations<R> {
    fn eq(&self, other: &Self) -> bool {
        self.r==other.r
    }
}
impl<R, O> Eq for NonZero<R, O> where
R: Ring<O>,
O: RingOperations<R> {

}
impl<R, O> Clone for NonZero<R, O> where
R: Ring<O>,
O: RingOperations<R> {
    fn clone(&self) -> Self {
        Self { r: self.r.clone(), o: PhantomData }
    }
}
impl<R, O> Set for NonZero<R, O> where
R: Ring<O>,
O: RingOperations<R>{

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
