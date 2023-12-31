use std::marker::PhantomData;

use crate::{
    structure::{euclidean_ring::{EuclideanRing}, ring::{RingOperations, Ideal, Ring}, monoid::{AbsorbingSubset, Monoid}, group::{Subgroup, Group}},
    wrapper::{Wrapper}, operation::O2, set::{Set, Subset},
};

pub struct Multiples<R:Ring<O>,O:RingOperations<R>,A:Wrapper<R>> {
    pub data: R,
    pub o:PhantomData<O>,
    pub a:PhantomData<A>
}
impl<R:Ring<O>,O:RingOperations<R>,A:Wrapper<R>> PartialEq for Multiples<R,O,A> {
    fn eq(&self, other: &Self) -> bool {
        self.data==other.data
    }
}
impl<R:Ring<O>,O:RingOperations<R>,A:Wrapper<R>> Eq for Multiples<R,O,A> {
    
}
impl<R:Ring<O>,O:RingOperations<R>,A:Wrapper<R>> Clone for Multiples<R,O,A> {
    fn clone(&self) -> Self {
        Self { data: self.data.clone(), o: PhantomData, a: PhantomData }
    }
}
impl<R:Ring<O>,O:RingOperations<R>,A:Wrapper<R>> Set for Multiples<R,O,A> {
    
}
impl<R:EuclideanRing<O>,O:RingOperations<R>,A:Wrapper<R>> Subset<R> for Multiples<R,O,A> {
    fn contains(t: &R) -> bool {
        t==&t.clone().quotient(&A::VAL()).times(&A::VAL())
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
impl<R:Ring<O>,O:RingOperations<R>,A:Wrapper<R>> O2<Multiples<R,O,A>> for O::PLUS {
    const F: fn(Multiples<R,O,A>, &Multiples<R,O,A>) -> Multiples<R,O,A> = |a,b| Multiples { data: a.data.plus(&b.data), o: PhantomData, a: PhantomData };
}
impl<R:Ring<O>,O:RingOperations<R>,A:Wrapper<R>> Monoid<O::PLUS> for Multiples<R,O,A> {
    fn identity() -> Self {
        Multiples { data: R::zero(), o: PhantomData, a: PhantomData }
    }
}
impl<R:Ring<O>,O:RingOperations<R>,A:Wrapper<R>> Group<O::PLUS> for Multiples<R,O,A> {
    fn inverse(self) -> Self {
        Multiples { data: self.data.inverse(), o: PhantomData, a: PhantomData }
    }
}
impl<R:EuclideanRing<O>,O:RingOperations<R>,A:Wrapper<R>> Subgroup<R,O::PLUS> for Multiples<R,O,A> {
    
}
impl<R:EuclideanRing<O>,O:RingOperations<R>,A:Wrapper<R>> AbsorbingSubset<R,O::TIMES> for Multiples<R,O,A> {
    fn times(self, m:&R) -> Self {
        Self { data: self.data.times(m), o: PhantomData, a: PhantomData }
    }
}
impl<R:EuclideanRing<O>,O:RingOperations<R>,A:Wrapper<R>> Ideal<R,O> for Multiples<R,O,A> {
    fn reduce(r: R) -> R {
        r.remainder(&A::VAL())
    }
}