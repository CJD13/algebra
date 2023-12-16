use crate::operation::{O2, Times, Plus};

use super::{group::{Group, Subgroup, AbelianGroup}, monoid::{Monoid, AbsorbingSubset}};

pub trait RingOperations<T>
where
    Self::PLUS: O2<T>,
    Self::TIMES: O2<T>,
{
    type PLUS;
    type TIMES;
}
pub trait Ring<O>: Group<O::PLUS> + Monoid<O::TIMES>
where
    O: RingOperations<Self>,
{
    //Any implementation must guarantee that TIMES distributes over PLUS
    fn zero() -> Self {
        <Self as Monoid<O::PLUS>>::identity()
    }
    fn one() -> Self {
        <Self as Monoid<O::TIMES>>::identity()
    }
    fn negated(self) -> Self {
        <Self as Group<O::PLUS>>::inverse(self)
    }
    fn plus(self, other: &Self) -> Self {
        O::PLUS::F(self, other)
    }
    //Override to eliminate the clone
    fn minus(self, other: &Self) -> Self {
        O::PLUS::F(self,&other.clone().negated())
    }
    fn times(self, other: &Self) -> Self {
        O::TIMES::F(self, other)
    }
    //Override to avoid the clone
    fn times_left(self, other: &Self) -> Self {
        other.clone().times(&self)
    }
    fn from_integer(n: u64) -> Self {
        Monoid::<O::PLUS>::pow(Self::one(),n)
    }
    fn pow(self, n:u64)->Self {
        <Self as Monoid<O::TIMES>>::pow(self, n)
    }
}
pub trait Ideal<R:Ring<O>,O:RingOperations<R>>: Subgroup<R,O::PLUS>+AbsorbingSubset<R,O::TIMES> where O::PLUS:O2<Self>{
    fn reduce(r: R) -> R {
        if Self::contains(&r) {
            R::zero()
        } else {
            r
        }
    }
}

impl<R, O, S, P> RingOperations<(R, S)> for (O, P)
where
    O: RingOperations<R>,
    R: Ring<O>,
    P: RingOperations<S>,
    S: Ring<P>,
{
    type PLUS = (O::PLUS, P::PLUS);
    type TIMES = (O::TIMES, P::TIMES);
}
impl<R, O, S, P> Ring<(O, P)> for (R, S)
where
    O: RingOperations<R>,
    R: Ring<O>,
    P: RingOperations<S>,
    S: Ring<P>,
{
}

/*struct ProductOps<R,O,S,P> where O:RingOperations<R>,R:Ring<O>, P:RingOperations<S>,S:Ring<P> {
    r:PhantomData<R>,
    o:PhantomData<O>,
    s:PhantomData<S>,
    p:PhantomData<P>
}*/

pub struct i64Ops {}
impl RingOperations<i64> for i64Ops {
    type PLUS = Plus;
    type TIMES = Times;
}
impl Ring<i64Ops> for i64 {}
