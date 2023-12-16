use std::marker::PhantomData;
use std::fmt::Debug;

use crate::{
    operation::{Plus, O2, Times},
    set::{Set, Subset},
    structure::{
        group::{Group, NormalSubgroup, Subgroup},
        monoid::{Monoid, AbsorbingSubset}, ring::{Ideal, RingOperations, Ring, i64Ops},
    }, wrapper::Wrapper, modular::Multiples,
};

#[derive(PartialEq, Eq, Clone)]
pub struct IntWrap<const N: i64> {}
impl<const N: i64> Wrapper<i64> for IntWrap<N> {
    const VAL: fn()->i64 = || N;
}
pub type IntMultiples<const N:i64>=Multiples<i64,i64Ops,IntWrap<N>>;
pub struct QuotientGroup<G, H, Op>
where
    G: Group<Op>,
    Op: O2<G> + O2<H>,
    H: NormalSubgroup<G, Op>,
{
    representative: G,
    h: PhantomData<H>,
    o: PhantomData<Op>,
}
impl<G, H, Op> Clone for QuotientGroup<G, H, Op>
where
    G: Group<Op>,
    Op: O2<G> + O2<H>,
    H: NormalSubgroup<G, Op>,
{
    fn clone(&self) -> Self {
        Self::from(self.representative.clone())
    }
}
impl<G, H, Op> PartialEq for QuotientGroup<G, H, Op>
where
    G: Group<Op>,
    Op: O2<G> + O2<H>,
    H: NormalSubgroup<G, Op>,
{
    fn eq(&self, other: &Self) -> bool {
        H::contains(
            &self
                .representative
                .clone().inverse()
                .star(&other.representative),
        )
    }
}
impl<G, H, Op> Eq for QuotientGroup<G, H, Op>
where
    G: Group<Op>,
    Op: O2<G> + O2<H>,
    H: NormalSubgroup<G, Op>,
{
}
impl<G, H, Op> Set for QuotientGroup<G, H, Op>
where
    G: Group<Op>,
    Op: O2<G> + O2<H>,
    H: NormalSubgroup<G, Op>,
{
}
impl<G, H, Op> O2<QuotientGroup<G, H, Op>> for Op
where
    G: Group<Op>,
    Op: O2<G> + O2<H>,
    H: NormalSubgroup<G, Op>,
{
    const F: fn(QuotientGroup<G, H, Op>, &QuotientGroup<G, H, Op>) -> QuotientGroup<G, H, Op> =
        |a, b| QuotientGroup::from(a.representative.star(&b.representative));
}
impl<G, H, Op> Monoid<Op> for QuotientGroup<G, H, Op>
where
    G: Group<Op>,
    Op: O2<G> + O2<H>,
    H: NormalSubgroup<G, Op>,
{
    fn identity() -> Self {
        QuotientGroup::from(G::identity())
    }
}
impl<G, H, Op> Group<Op> for QuotientGroup<G, H, Op>
where
    G: Group<Op>,
    Op: O2<G> + O2<H>,
    H: NormalSubgroup<G, Op>,
{
    fn inverse(self) -> Self {
        QuotientGroup::from(self.representative.inverse())
    }
}
impl<G, H, Op> Debug for QuotientGroup<G, H, Op>
where
    G: Group<Op>,
    Op: O2<G> + O2<H>,
    H: NormalSubgroup<G, Op>,
    G: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}]", self.representative)
    }
}
impl<G, H, Op> From<G> for QuotientGroup<G, H, Op>
where
    G: Group<Op>,
    Op: O2<G> + O2<H>,
    H: NormalSubgroup<G, Op>,
{
    //this is a group homomorphism from G to Self with kernel H
    fn from(g: G) -> Self {
        Self {
            representative: H::reduce(g),
            h: PhantomData,
            o: PhantomData,
        }
    }
}
pub struct QuotientRing<R:Ring<O>,O:RingOperations<R>,I:Ideal<R,O>> where O::PLUS:O2<I>{
    representative: R,
    i: PhantomData<I>,
    o: PhantomData<O>
}
impl<R:Ring<O>,O:RingOperations<R>,I:Ideal<R,O>> From<R> for QuotientRing<R,O,I>  where O::PLUS:O2<I>{
    fn from(value: R) -> Self {
        Self { representative: I::reduce(value), i: PhantomData, o: PhantomData }
    }
}
impl<R:Ring<O>,O:RingOperations<R>,I:Ideal<R,O>> Clone for QuotientRing<R,O,I> where O::PLUS:O2<I>{
    fn clone(&self) -> Self {
        Self::from(self.representative.clone())
    }
}
impl<R:Ring<O>,O:RingOperations<R>,I:Ideal<R,O>> PartialEq for QuotientRing<R,O,I> where O::PLUS:O2<I>{
    fn eq(&self, other: &Self) -> bool {
        I::contains(&self.representative.clone().negated().plus(&other.representative))
    }
}
impl<R:Ring<O>,O:RingOperations<R>,I:Ideal<R,O>> Eq for QuotientRing<R,O,I> where O::PLUS:O2<I>{

}
impl<R:Ring<O>,O:RingOperations<R>,I:Ideal<R,O>> Set for QuotientRing<R,O,I> where O::PLUS:O2<I>{

}
pub struct QuotientSum<O>{
    o:PhantomData<O>
}
pub struct QuotientProduct<O> {
    o:PhantomData<O>
}
impl<R:Ring<O>,O:RingOperations<R>,I:Ideal<R,O>> RingOperations<QuotientRing<R,O,I>> for O where O::PLUS:O2<I>{
    type PLUS = QuotientSum<O::PLUS>;
    type TIMES = QuotientProduct<O::TIMES>;
}
impl<R:Ring<O>,O:RingOperations<R>,I:Ideal<R,O>> O2<QuotientRing<R,O,I>> for QuotientSum<O::PLUS> where O::PLUS:O2<I>{
    const F: fn(QuotientRing<R,O,I>, &QuotientRing<R,O,I>) -> QuotientRing<R,O,I> = |a,b| QuotientRing::from(a.representative.plus(&b.representative));
}
impl<R:Ring<O>,O:RingOperations<R>,I:Ideal<R,O>> O2<QuotientRing<R,O,I>> for QuotientProduct<O::TIMES> where O::PLUS:O2<I>{
    const F: fn(QuotientRing<R,O,I>, &QuotientRing<R,O,I>) -> QuotientRing<R,O,I> = |a,b| QuotientRing::from(a.representative.times(&b.representative));
}
impl<R:Ring<O>,O:RingOperations<R>,I:Ideal<R,O>> Monoid<QuotientSum<O::PLUS>> for QuotientRing<R,O,I> where O::PLUS:O2<I>{
    fn identity() -> Self {
        Self::from(R::zero())
    }
}
impl<R:Ring<O>,O:RingOperations<R>,I:Ideal<R,O>> Group<QuotientSum<O::PLUS>> for QuotientRing<R,O,I> where O::PLUS:O2<I>{
    fn inverse(self) -> Self {
        Self::from(self.representative.inverse())
    }
}
impl<R:Ring<O>,O:RingOperations<R>,I:Ideal<R,O>> Monoid<QuotientProduct<O::TIMES>> for QuotientRing<R,O,I> where O::PLUS:O2<I>{
    fn identity() -> Self {
        Self::from(R::one())
    }
}
impl<R:Ring<O>,O:RingOperations<R>,I:Ideal<R,O>> Ring<O> for QuotientRing<R,O,I> where O::PLUS:O2<I>{

}
impl<R:Ring<O>,O:RingOperations<R>,I:Ideal<R,O>> Debug for QuotientRing<R,O,I> where O::PLUS:O2<I>,R:Debug{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}]", self.representative)
    }
}
