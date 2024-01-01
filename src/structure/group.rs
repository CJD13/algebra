use crate::{operation::{O2}, set::Subset};

use super::monoid::Monoid;

/// In addition to the properties required by the [`Monoid`] trait, any implementation must
/// guarantee that the product of any element with its inverse gives the identity.
pub trait Group<Operation: O2<Self>>: Monoid<Operation> {
    /// For a implementation to be correct, for every element `a` in a group `G`, `Operation::F(a,
    /// a.inverse())` should be `==` to `G::identity()`.
    fn inverse(self) -> Self;
    fn pow(a: Self, n: i64) -> Self {
        if n > 0 {
            return <Self as Monoid<Operation>>::pow(a, n as u64);
        }
        <Self as Monoid<Operation>>::pow(a, -n as u64).inverse()
    }
}
pub trait AbelianGroup<Operation: O2<Self>>: Group<Operation> {
    //Only implement this trait for abelian groups.
    //A group G is abelian if gh=hg for all g, h in G.
}
pub trait Subgroup<G, Operation: O2<G>>: Subset<G> + Group<Operation>
where
    G: Group<Operation>,
    Operation: O2<Self>,
{
}
pub trait NormalSubgroup<G, Operation: O2<G>>: Subgroup<G, Operation>
where
    G: Group<Operation>,
    Operation: O2<Self>,
{
    //Only implement this trait for normal subgroups.
    //A subgroup H of a group G is normal if for all h in H and g in G, ghg^{-1} is an element of H.
    //When performing operations in a quotient group it can improve efficiency to replace representatives with simpler equivalent representatives.
    //The prototypical example is the % operator; applying this repeatedly when working mod n
    //keeps numbers small and computations fast.
    //Override this with a good reduction method.
    fn reduce(g: G) -> G {
        if Self::contains(&g) {
            G::identity()
        } else {
            g
        }
    }
}
impl<G: AbelianGroup<Op>, Op: O2<G>, S: Subgroup<G, Op>> NormalSubgroup<G, Op> for S
where
    Op: O2<S>,
{
    //Any subgroup of an abelian group is normal.
}

impl<M1, T1: O2<M1>, M2, T2: O2<M2>> Group<(T1, T2)> for (M1, M2)
where
    M1: Group<T1>,
    M2: Group<T2>,
{
    fn inverse(self) -> Self {
        (self.0.inverse(), self.1.inverse())
    }
}

