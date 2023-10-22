use crate::{operation::{O2, i64Times, i64Plus}, polynomial::Degree};

use super::{group::{Group, Subgroup, AbelianGroup}, monoid::{Monoid, AbsorbingSubset}, ring::{Ring, RingOperations, i64Ops}};
pub trait EuclideanRing<O:RingOperations<Self>>:Ring<O> {
    //The Euclidean norm of the ring.
    fn norm(&self) -> Degree;
    //Division. The result q must have the property that norm(self-q*divisor)<norm(divisor).
    //There is the added requirement that if divisor divides self, then self-q*divisor must equal zero.
    //Division by zero is not defined.
    fn divide(self,divisor:Self) -> Self;
}
impl EuclideanRing<i64Ops> for i64 {
    fn norm(&self) -> Degree {
        Degree::Integer(self.abs() as usize)
    }
    fn divide(self,divisor:Self) -> Self {
        self/divisor
    }
}
