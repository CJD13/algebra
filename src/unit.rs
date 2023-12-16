use std::marker::PhantomData;

use crate::{structure::{ring::{Ring, RingOperations}, group::Group, monoid::Monoid, euclidean_ring::EuclideanRing}, operation::O2, set::{Set, Subset}};

pub struct Unit<R:Ring<O>,O:RingOperations<R>> {
    u: R,
    u_inverse: R,
    o:PhantomData<O>
}
impl<R:Ring<O>,O:RingOperations<R>> O2<Unit<R,O>> for O::TIMES {
    const F: fn(Unit<R,O>, &Unit<R,O>) -> Unit<R,O> = |a,b| Unit { u: a.u.times(&b.u), u_inverse: a.u_inverse.times_left(&b.u_inverse), o: PhantomData };
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

pub trait TryInverse<O:RingOperations<Self>>: Ring<O> {
    //This must return the multiplicative inverse of self, or None if no such inverse exists.
    fn try_inverse(self)->Option<Self>;
}
impl<R:EuclideanRing<O>,O:RingOperations<R>> TryInverse<O> for R {
    fn try_inverse(self)->Option<Self> {
        let (q,r)=R::one().divide(&self);
        if r==R::zero() {
            Some(q)
        } else {
            None
        }
    }
}
impl<R:Ring<O>,O:RingOperations<R>> Subset<R> for Unit<R,O> where R:TryInverse<O> {
    fn contains(t: &R) -> bool {
        t.clone().try_inverse().is_some()
    }
    fn inclusion(self) -> R {
        self.u
    }
    fn try_from(t: R) -> Self {
        Unit { u: t.clone(), u_inverse: t.try_inverse().unwrap(),o:PhantomData}
    }
}