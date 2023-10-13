use crate::operation::{O2, i64Times, i64Plus};

use super::{group::Group, monoid::Monoid};

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
    fn plus(self, other: Self) -> Self {
        O::PLUS::F(self, other)
    }
    fn times(self, other: Self) -> Self {
        O::TIMES::F(self, other)
    }
    fn times_integer(self, other: i64) -> Self {
        Ring::times(self, Group::pow(Self::one(), other))
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
    type PLUS = i64Plus;
    type TIMES = i64Times;
}
impl Ring<i64Ops> for i64 {}
