use std::marker::PhantomData;

use crate::{
    structure::{euclidean_ring::{EuclideanRing}, ring::{RingOperations, Ideal, Ring}, monoid::{AbsorbingSubset, Monoid}, group::{Subgroup, Group}},
    wrapper::{Wrapper}, operation::O2, set::{Set, Subset},
};

struct Multiples<R:EuclideanRing<O>,O:RingOperations<R>,A:Wrapper<R>> {
    data: R,
    o:PhantomData<O>,
    a:PhantomData<A>
}
impl<R:EuclideanRing<O>,O:RingOperations<R>,A:Wrapper<R>> PartialEq for Multiples<R,O,A> {
    fn eq(&self, other: &Self) -> bool {
        self.data==other.data
    }
}
impl<R:EuclideanRing<O>,O:RingOperations<R>,A:Wrapper<R>> Eq for Multiples<R,O,A> {
    
}
impl<R:EuclideanRing<O>,O:RingOperations<R>,A:Wrapper<R>> Clone for Multiples<R,O,A> {
    fn clone(&self) -> Self {
        Self { data: self.data.clone(), o: PhantomData, a: PhantomData }
    }
}
impl<R:EuclideanRing<O>,O:RingOperations<R>,A:Wrapper<R>> Set for Multiples<R,O,A> {
    
}
impl<R:EuclideanRing<O>,O:RingOperations<R>,A:Wrapper<R>> Subset<R> for Multiples<R,O,A> {
    fn contains(t: &R) -> bool {
        t==&Ring::times(t.clone().divide(A::Val),A::Val)
    }
    fn inclusion(self) -> R {
        self.data
    }
    fn try_from(t: R) -> Self {
        if Self::contains(&t) {
            Self {data: t,o:PhantomData,a:PhantomData}
        } else {
            panic!()
        }
    }
}
impl<R:EuclideanRing<O>,O:RingOperations<R>,A:Wrapper<R>> O2<Multiples<R,O,A>> for O::PLUS {
    const F: fn(Multiples<R,O,A>, Multiples<R,O,A>) -> Multiples<R,O,A> = |a,b| Multiples { data: a.data.plus(b.data), o: PhantomData, a: PhantomData };
}
impl<R:EuclideanRing<O>,O:RingOperations<R>,A:Wrapper<R>> Monoid<O::PLUS> for Multiples<R,O,A> {
    fn identity() -> Self {
        Multiples { data: R::zero(), o: PhantomData, a: PhantomData }
    }
}
impl<R:EuclideanRing<O>,O:RingOperations<R>,A:Wrapper<R>> Group<O::PLUS> for Multiples<R,O,A> {
    fn inverse(self) -> Self {
        Multiples { data: self.data.inverse(), o: PhantomData, a: PhantomData }
    }
}
impl<R:EuclideanRing<O>,O:RingOperations<R>,A:Wrapper<R>> Subgroup<R,O::PLUS> for Multiples<R,O,A> {
    
}
impl<R:EuclideanRing<O>,O:RingOperations<R>,A:Wrapper<R>> AbsorbingSubset<R,O::TIMES> for Multiples<R,O,A> {
    fn times(self, m:R) -> Self {
        Self { data: Ring::times(self.data,m), o: PhantomData, a: PhantomData }
    }
}
impl<R:EuclideanRing<O>,O:RingOperations<R>,A:Wrapper<R>> Ideal<R,O> for Multiples<R,O,A> {
    
}