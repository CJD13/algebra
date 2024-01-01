
use crate::{operation::{O2}, polynomial::Degree, nonzero::NonZero, impls::StandardOps};

use super::{group::{Group, Subgroup, AbelianGroup}, monoid::{Monoid, AbsorbingSubset}, ring::{Ring, RingOperations}, field::Field};
pub trait EuclideanRing<O:RingOperations<Self>>:Ring<O> {
    //The Euclidean norm of the ring.
    fn norm(&self) -> Degree;
    //Division. The result q must have the property that norm(self-q*divisor)<norm(divisor).
    //There is the added requirement that if divisor divides self, then self-q*divisor must equal zero.
    //Division by zero is not defined.
    fn quotient(self,divisor:&Self) -> Self;
    fn remainder(self,divisor:&Self) -> Self {
        self.clone().plus(&self.quotient(divisor).times(divisor)).negated()
    }
    fn divide(self, divisor:&Self) -> (Self,Self) {
        (self.clone().quotient(divisor),self.remainder(divisor))
    }
    //Returns x and y such that ax+by=gcd(a,b)
    fn bézout(a:Self,b:Self)->(Self,Self){
        if b==Self::zero() {
            (Self::one(),Self::zero())
        } else {
            let (q,r) = a.divide(&b);
            let (x,y)=Self::bézout(b,r);
            let s = x.minus(&q.times(&y));
            (y,s)
        }
    }
    fn gcd(a:Self,b:Self) -> Self {
        if b==Self::zero() {
            a
        } else {
            let (q,r) = a.divide(&b);
            Self::gcd(b,r)
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
    fn quotient(self,divisor:&Self) -> Self {
        self.times(&divisor.clone().reciprocal())
    }
    fn remainder(self,divisor:&Self) -> Self {
        Self::zero()
    }
}
impl EuclideanRing<StandardOps> for i64 {
    fn norm(&self) -> Degree {
        Degree::Integer(self.abs() as usize)
    }
    fn quotient(self,divisor:&Self) -> Self {
        self.div_euclid(*divisor)
    }
    fn remainder(self,divisor:&Self) -> Self {
        self%divisor
    }
}
