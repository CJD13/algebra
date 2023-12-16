use std::{marker::PhantomData, ops::{Add, Mul}, cmp::Ordering};
use crate::{structure::{ring::{RingOperations, Ring, i64Ops, Ideal}, group::{Group, Subgroup}, monoid::{Monoid, AbsorbingSubset}, field::Field, euclidean_ring::EuclideanRing}, set::{Set, Subset}, operation::O2, nonzero::NonZero, unit::TryInverse, wrapper::Wrapper, modular::Multiples, quotient::QuotientRing};
use take_mut::take;
pub struct Polynomial<R, O: RingOperations<R>>
where
    R: Ring<O>,
{
    pub coefficients: Vec<R>,
    pub o: PhantomData<O>,
}
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub enum Degree {
    NegInfty,
    Integer(usize),
}
impl Degree {
    /// Panics if the degree is `Degree::NegInfty`.
    fn unwrap(self) -> usize {
        match self {
            Self::Integer(n) => n,
            Self::NegInfty => panic!(),
        }
    }
}
impl PartialEq<usize> for Degree {
    fn eq(&self, other: &usize) -> bool {
        match *self {
            Degree::NegInfty => false,
            Degree::Integer(n) => &n == other,
        }
    }
}
impl PartialOrd<usize> for Degree {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        match *self {
            Degree::NegInfty => Some(Ordering::Less),
            Degree::Integer(n) => n.partial_cmp(other),
        }
    }
}
impl Add<usize> for Degree {
    type Output = Degree;
    fn add(self, rhs: usize) -> Self::Output {
        match self {
            Degree::NegInfty => Degree::NegInfty,
            Degree::Integer(n) => Degree::Integer(n + rhs),
        }
    }
}
impl Add<Degree> for Degree {
    type Output = Degree;
    fn add(self, other: Degree) -> Degree {
        match self {
            Degree::NegInfty => Degree::NegInfty,
            Degree::Integer(n) => other + n,
        }
    }
}
impl<R, O> Polynomial<R, O>
where
    O: RingOperations<R>,
    R: Ring<O>,
{
    fn trim_zeros(mut v: Vec<R>) -> Vec<R> {
        if !v.is_empty() {
            let mut k = v.pop().unwrap();
            while k == R::zero() && !v.is_empty() {
                k = v.pop().unwrap();
            }
            if k != R::zero() {
                v.push(k)
            }
        }
        v
    }
    fn zero() -> Self {
        Polynomial {
            coefficients: vec![],
            o: PhantomData,
        }
    }
    fn one() -> Self {
        Polynomial {
            coefficients: vec![R::one()],
            o: PhantomData,
        }
    }
    fn x() -> Self {
        Polynomial {
            coefficients: vec![R::one(), R::zero()],
            o: PhantomData,
        }
    }
    fn constant(c: R) -> Self {
        Polynomial {
            coefficients: Self::trim_zeros(vec![c]),
            o: PhantomData,
        }
    }
    fn degree(&self) -> Degree {
        if self.coefficients.is_empty() {
            Degree::NegInfty
        } else {
            Degree::Integer(self.coefficients.len() - 1)
        }
    }
    fn coefficient(&self, n: usize) -> R {
        if self.degree() >= n {
            self.coefficients[n].clone()
        } else {
            R::zero()
        }
    }
    fn add(mut self, other: &Self) -> Self {
        if self.degree()<other.degree() {
            return Self::add(other.clone(),&self)
        }
        let mut l = 0;
        while other.degree()>=l {
            take(&mut self.coefficients[l],|c| c.plus(&other.coefficients[l]));
            l+=1;
        }
        self.coefficients=Self::trim_zeros(self.coefficients);
        self
    }
    fn mul(self, other: &Self) -> Self {
        let l = self.degree() + other.degree();
        let mut res = vec![];
        let mut i = 0;
        while l >= i {
            let mut s = R::zero();
            for j in 0..=i {
                s = Ring::plus(
                    s,
                    &self.coefficient(j).times( &other.coefficient(i - j)),
                );
            }
            res.push(s);
            i += 1;
        }
        res = Self::trim_zeros(res);
        Polynomial {
            coefficients: res,
            o: PhantomData,
        }
    }

    fn negated(self) -> Self {
        let mut res = vec![];
        for i in 0..self.coefficients.len() {
            res.push(self.coefficients[i].clone().negated());
        }
        Polynomial {
            coefficients: res,
            o: PhantomData,
        }
    }
    //fn derivative(self)->Self {
    //self
    //}
    pub fn of(&self, x: R) -> R {
        let mut n = 0;
        let mut res = R::zero();
        let mut xPower = R::one();
        while self.degree() >= n {
            res = res.plus(&self.coefficient(n).times( &xPower));
            xPower = xPower.times( &x);
            n += 1;
        }
        res
    }
    /// Gives the leading coefficient of self. The result is guaranteed to be nonzero. Panics if
    /// given the zero polynomial, which has no leading coefficient.
    fn lead_coeff(&self) -> R {
        match self.degree() {
            Degree::Integer(n) => self.coefficient(n),
            Degree::NegInfty => panic!(),
        }
    }
    fn x_pow(n:usize) -> Self{
        let mut data = Vec::with_capacity(n+1);
        for _ in 0..n {
            data.push(R::zero());
        }
        data.push(R::one());
        Polynomial { coefficients: data, o: PhantomData }
    }
}
impl<R:Ring<O>, O:RingOperations<R>> Polynomial<R, O>
where R:TryInverse<O>
{
    /// The leading coefficient of the divisor must be a unit.
    /// Panics if this is not the case.
    fn divide(mut dividend: Self, divisor: &Self) -> (Self, Self) {
        let n = divisor.degree().unwrap();
        let i = divisor.lead_coeff().try_inverse().unwrap();
        let mut quotient = Self::zero();
        while dividend.degree() >= divisor.degree() {
            let m = dividend.degree().unwrap();
            let term = 
                Self::constant(dividend.lead_coeff().times(&i)).times(
                &Self::x_pow(m - n));
            quotient = quotient + term.clone();
            dividend = dividend + (divisor.clone() * term).negated();
        }
        (quotient, dividend)
    }
}
impl<R, O> Add<Polynomial<R, O>> for Polynomial<R, O>
where
    O: RingOperations<R>,
    R: Ring<O>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.add(&rhs)
    }
}
impl<R, O> Mul<Polynomial<R, O>> for Polynomial<R, O>
where
    O: RingOperations<R>,
    R: Ring<O>,
{
    type Output = Self;
    fn mul(self, rhs: Polynomial<R, O>) -> Self::Output {
        self.mul(&rhs)
    }
}
struct PPLUS<R, O>
where
    O: RingOperations<R>,
    R: Ring<O>,
{
    r: PhantomData<R>,
    o: PhantomData<O>,
}
struct PTIMES<R, O>
where
    O: RingOperations<R>,
    R: Ring<O>,
{
    r: PhantomData<R>,
    p: PhantomData<O>,
}
impl<R, O> O2<Polynomial<R, O>> for PPLUS<R, O>
where
    O: RingOperations<R>,
    R: Ring<O>,
{
    const F: fn(Polynomial<R, O>, &Polynomial<R, O>) -> Polynomial<R, O> = <Polynomial<R, O>>::add;
}
impl<R, O> O2<Polynomial<R, O>> for PTIMES<R, O>
where
    O: RingOperations<R>,
    R: Ring<O>,
{
    const F: fn(Polynomial<R, O>, &Polynomial<R, O>) -> Polynomial<R, O> = <Polynomial<R, O>>::mul;
}

impl<R, O> PartialEq for Polynomial<R, O>
where
    O: RingOperations<R>,
    R: Ring<O>,
{
    fn eq(&self, other: &Self) -> bool {
        self.coefficients == other.coefficients
    }
}
impl<R, O> Eq for Polynomial<R, O>
where
    O: RingOperations<R>,
    R: Ring<O>,
{
}
impl<R, O> Clone for Polynomial<R, O>
where
    O: RingOperations<R>,
    R: Ring<O>,
{
    fn clone(&self) -> Self {
        Polynomial {
            coefficients: self.coefficients.clone(),
            o: PhantomData,
        }
    }
}
impl<R, O> Set for Polynomial<R, O>
where
    O: RingOperations<R>,
    R: Ring<O>,
{
}
impl<R, O> Monoid<PTIMES<R, O>> for Polynomial<R, O>
where
    O: RingOperations<R>,
    R: Ring<O>,
{
    fn identity() -> Self {
        Self::one()
    }
}
impl<R, O> Monoid<PPLUS<R, O>> for Polynomial<R, O>
where
    O: RingOperations<R>,
    R: Ring<O>,
{
    fn identity() -> Self {
        Self::zero()
    }
}
impl<R, O> Group<PPLUS<R, O>> for Polynomial<R, O>
where
    O: RingOperations<R>,
    R: Ring<O>,
{
    fn inverse(self) -> Self {
        self.negated()
    }
}
struct PolyOps<R, O>
where
    O: RingOperations<R>,
    R: Ring<O>,
{
    r: PhantomData<R>,
    o: PhantomData<O>,
}
impl<R, O> RingOperations<Polynomial<R, O>> for PolyOps<R, O>
where
    O: RingOperations<R>,
    R: Ring<O>,
{
    type PLUS = PPLUS<R, O>;
    type TIMES = PTIMES<R, O>;
}
impl<R, O> Ring<PolyOps<R, O>> for Polynomial<R, O>
where
    O: RingOperations<R>,
    R: Ring<O>,
{
}
impl<F, O> EuclideanRing<PolyOps<F,O>> for Polynomial<F, O>
where
    O: RingOperations<F>,
    F: Field<O>,
    O::TIMES: O2<NonZero<F, O>>,
    NonZero<F, O>: Group<O::TIMES>,
{
    fn norm(&self) -> Degree {
        self.degree()
    }
    fn quotient(self,divisor:&Self) -> Self {
        Polynomial::divide(self,divisor).0
    }
    fn divide(self, divisor:&Self) -> (Self,Self) {
        Self::divide(self, divisor)
    }
    fn remainder(self,divisor:&Self) -> Self {
        Self::divide(self, divisor).1
    }
}
impl<P:Wrapper<Polynomial<i64,i64Ops>>> Subset<Polynomial<i64,i64Ops>> for Multiples<Polynomial<i64,i64Ops>,PolyOps<i64,i64Ops>,P> {
    fn contains(t: &Polynomial<i64,i64Ops>) -> bool {
        Polynomial::divide(t.clone(), &P::VAL()).1==Polynomial::zero()
    }
    fn inclusion(self) -> Polynomial<i64,i64Ops> {
        self.data
    }
    fn try_from(t: Polynomial<i64,i64Ops>) -> Self {
        if Self::contains(&t) {
            Self {data: t,o:PhantomData,a:PhantomData}
        } else {
            panic!()
        }
    }
}
impl<P:Wrapper<Polynomial<i64,i64Ops>>> Subgroup<Polynomial<i64,i64Ops>,PPLUS<i64,i64Ops>> for Multiples<Polynomial<i64,i64Ops>,PolyOps<i64,i64Ops>,P> {
    
}
impl<P:Wrapper<Polynomial<i64,i64Ops>>> AbsorbingSubset<Polynomial<i64,i64Ops>,PTIMES<i64,i64Ops>> for Multiples<Polynomial<i64,i64Ops>,PolyOps<i64,i64Ops>,P> {
    
}

impl<P:Wrapper<Polynomial<i64,i64Ops>>> Ideal<Polynomial<i64,i64Ops>,PolyOps<i64,i64Ops>> for Multiples<Polynomial<i64,i64Ops>,PolyOps<i64,i64Ops>,P> {
    
}
type i64AdjX=Polynomial<i64,i64Ops>;
struct XSquaredPlus1;
impl Wrapper<i64AdjX> for XSquaredPlus1 {
    const VAL: fn()->i64AdjX = || i64AdjX::x().times(&i64AdjX::x()).plus(&i64AdjX::one());
}
type GaussianI64=QuotientRing<i64AdjX,PolyOps<i64,i64Ops>,Multiples<i64AdjX,PolyOps<i64,i64Ops>,XSquaredPlus1>>;