use std::marker::PhantomData;
use std::fmt::Debug;

use crate::{
    operation::{i64Plus, O2},
    set::{Set, Subset},
    structure::{
        group::{Group, NormalSubgroup, Subgroup},
        monoid::Monoid,
    },
};

#[derive(PartialEq, Eq, Clone)]
pub struct Multiples<const N: i64> {
    data: i64,
}
impl<const N: i64> O2<Multiples<N>> for i64Plus {
    const F: fn(Multiples<N>, Multiples<N>) -> Multiples<N> = |a, b| Multiples {
        data: a.data + b.data,
    };
}
impl<const N: i64> Set for Multiples<N> {}
impl<const N: i64> Subset<i64> for Multiples<N> {
    fn contains(t: &i64) -> bool {
        t % N == 0
    }
    fn inclusion(self) -> i64 {
        self.data
    }
    fn try_from(t: i64) -> Self {
        if Self::contains(&t) {
            Self { data: t }
        } else {
            panic!()
        }
    }
}
impl<const N: i64> Monoid<i64Plus> for Multiples<N> {
    fn identity() -> Self {
        Self { data: 0 }
    }
}
impl<const N: i64> Group<i64Plus> for Multiples<N> {
    fn inverse(self) -> Self {
        Self { data: -self.data }
    }
}
impl<const N: i64> Subgroup<i64, i64Plus> for Multiples<N> {}
impl<const N: i64> NormalSubgroup<i64, i64Plus> for Multiples<N> {
    fn reduce(g: i64) -> i64 {
        g % N
    }
}
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
        Self {
            representative: self.representative.clone(),
            h: PhantomData,
            o: PhantomData,
        }
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
                .clone()
                .times(other.representative.clone().inverse()),
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
    const F: fn(QuotientGroup<G, H, Op>, QuotientGroup<G, H, Op>) -> QuotientGroup<G, H, Op> =
        |a, b| QuotientGroup {
            representative: H::reduce(a.representative.times(b.representative)),
            h: PhantomData,
            o: PhantomData,
        };
}
impl<G, H, Op> Monoid<Op> for QuotientGroup<G, H, Op>
where
    G: Group<Op>,
    Op: O2<G> + O2<H>,
    H: NormalSubgroup<G, Op>,
{
    fn identity() -> Self {
        QuotientGroup {
            representative: G::identity(),
            h: PhantomData,
            o: PhantomData,
        }
    }
}
impl<G, H, Op> Group<Op> for QuotientGroup<G, H, Op>
where
    G: Group<Op>,
    Op: O2<G> + O2<H>,
    H: NormalSubgroup<G, Op>,
{
    fn inverse(self) -> Self {
        QuotientGroup {
            representative: self.representative.inverse(),
            h: PhantomData,
            o: PhantomData,
        }
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
impl<G, H, Op> QuotientGroup<G, H, Op>
where
    G: Group<Op>,
    Op: O2<G> + O2<H>,
    H: NormalSubgroup<G, Op>,
{
    //this is a group homomorphism from G to Self with kernel H
    pub fn natural_projection(g: G) -> Self {
        Self {
            representative: H::reduce(g),
            h: PhantomData,
            o: PhantomData,
        }
    }
}
