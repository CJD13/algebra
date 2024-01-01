use crate::{operation::{O2}, set::{Set, Subset}};

///Any implementation must guarantee that:
/// * the multiplication `Operation` is associative, and
/// * `Monoid::identity` returns the identity of the multiplication.
pub trait Monoid<Operation: O2<Self>>: Set {
    fn identity() -> Self;
    /// The correctness of the default implementation of this function relies on the properties
    /// required by this trait.
    fn pow(self, n: u64) -> Self {
        if n == 0 {
            return Self::identity();
        }
        let rem = if n % 2 == 1 {
            self.clone()
        } else {
            Self::identity()
        };
        let i = self.pow(n / 2);
        Operation::F(Operation::F(i.clone(), &i), &rem)
    }
    fn star(self, other: &Self) -> Self {
        Operation::F(self, other)
    }
}
pub trait Submonoid<M,O:O2<M>>:Subset<M>+Monoid<O> where M: Monoid<O>,O:O2<Self> {

}
pub trait AbsorbingSubset<M,O:O2<M>>:Subset<M> where M: Monoid<O>{
    //An absorbing subset A of M has the property that am and ma are  in A for all a in A, m in M
    //An example is the zero subset of the multiplicative monoid of any ring
    //implementations must guarantee that the try_from never panics
    fn times(self, m:&M) -> Self {
        Self::try_from(self.inclusion().star(m))
    }
}



impl<M1, T1: O2<M1>, M2, T2: O2<M2>> Monoid<(T1, T2)> for (M1, M2)
where
    M1: Monoid<T1>,
    M2: Monoid<T2>,
{
    fn identity() -> Self {
        (M1::identity(), M2::identity())
    }
}
