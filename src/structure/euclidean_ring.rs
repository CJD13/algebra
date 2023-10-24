
use crate::{operation::{O2, i64Times, i64Plus}, polynomial::Degree, nonzero::NonZero};

use super::{group::{Group, Subgroup, AbelianGroup}, monoid::{Monoid, AbsorbingSubset}, ring::{Ring, RingOperations, i64Ops}, field::Field};
pub trait EuclideanRing<O:RingOperations<Self>>:Ring<O> {
    //The Euclidean norm of the ring.
    fn norm(&self) -> Degree;
    //Division. The result q must have the property that norm(self-q*divisor)<norm(divisor).
    //There is the added requirement that if divisor divides self, then self-q*divisor must equal zero.
    //Division by zero is not defined.
    fn divide(self,divisor:Self) -> Self;
    fn remainder(self,divisor:Self) -> Self {
        self.clone().plus(divisor.clone().times(self.divide(divisor)).negated())
    }
    //Returns x and y such that ax+by=gcd(a,b)
    fn bézout(a:Self,b:Self)->(Self,Self){
        if b==Self::zero() {
            (Self::one(),Self::zero())
        } else {
            let q = a.clone().divide(b.clone());
            let (x,y)=Self::bézout(b.clone(),a.plus(q.clone().times(b).negated()));
            (y.clone(),x.plus(q.times( y).negated()))
        }
    }
    fn gcd(a:Self,b:Self) -> Self {
        if b==Self::zero() {
            a
        } else {
            Self::gcd(b.clone(),a.remainder(b))
        }
    }
}
impl<F:Field<O>,O:RingOperations<F>> EuclideanRing<O> for F where O::TIMES: O2<NonZero<F, O>>,NonZero<Self, O>: Group<O::TIMES>{
    fn norm(&self) -> Degree {
        if self==&F::zero(){
            Degree::NegInfty
        } else {
            Degree::Integer(0)
        }
    }
    fn divide(self,divisor:Self) -> Self {
        self.times(Field::inverse(divisor))
    }
    fn remainder(self,divisor:Self) -> Self {
        Self::zero()
    }
}
impl EuclideanRing<i64Ops> for i64 {
    fn norm(&self) -> Degree {
        Degree::Integer(self.abs() as usize)
    }
    fn divide(self,divisor:Self) -> Self {
        self.div_euclid(divisor)
    }
    fn remainder(self,divisor:Self) -> Self {
        self%divisor
    }
}
