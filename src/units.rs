use std::marker::PhantomData;

use crate::{structure::{ring::{Ring, RingOperations}, group::Group, monoid::Monoid}, operation::O2, set::Set};

pub struct Unit<R:Ring<O>,O:RingOperations<R>> {
    u: R,
    u_inverse: R,
    o:PhantomData<O>
}
impl<R:Ring<O>,O:RingOperations<R>> O2<Unit<R,O>> for O::TIMES {
    const F: fn(Unit<R,O>, Unit<R,O>) -> Unit<R,O> = |a,b| Unit { u: a.u.times(b.u), u_inverse: b.u_inverse.times(a.u_inverse), o: PhantomData };
}
impl<R:Ring<O>,O:RingOperations<R>> PartialEq for Unit<R,O> {
    fn eq(&self, other: &Self) -> bool {
        self.u==other.u
    }
}
impl<R:Ring<O>,O:RingOperations<R>> Eq for Unit<R,O> {
    
}
impl<R:Ring<O>,O:RingOperations<R>> Clone for Unit<R,O> {
    fn clone(&self) -> Self {
        Unit { u: self.u.clone(), u_inverse: self.u_inverse.clone(), o: PhantomData }
    }
}
impl<R:Ring<O>,O:RingOperations<R>> Set for Unit<R,O> {
    
}
impl<R:Ring<O>,O:RingOperations<R>> Monoid<O::TIMES> for Unit<R,O> {
    fn identity() -> Self {
        Unit { u: R::one(), u_inverse: R::one(), o: PhantomData }
    }
}
impl<R:Ring<O>,O:RingOperations<R>> Group<O::TIMES> for Unit<R,O> {
    fn inverse(self) -> Self {
        Unit { u: self.u_inverse, u_inverse: self.u, o: PhantomData }
    }
}