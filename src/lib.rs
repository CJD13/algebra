use core::panic;
use std::cmp::Ordering;
use std::ops::{Mul, Add};
use std::marker::PhantomData;
//use num::BigInt;
mod test;
pub trait Subset<T> where T:Set {
    /// A subset of `T` is determined by which elements of `T` are members of it.
    fn contains(t:&T) -> bool;
    /// Any object of type `Self` can be coerced into one of type `T`
    fn inclusion(self) -> T;
    /// Some `T`s can be turned into `Self`s
    fn try_from(t:T) -> Self;
}

/// A finite subset can be iterated over. An implementation must guarantee that each member t of T
/// is `==` to exactly one item of the iterator if Self::contains(t) is true and does not == any if
/// it is false.
pub trait FiniteSubset<T>:Subset<T>+IntoIterator<Item=T> where T:Set{
    const ORDER: usize;
}
/// A struct implementing FiniteSet must have ORDER equal to the number of objects of that struct.
/// It must also be able to provide an iterator over its elements. Any member of the struct must be
/// == to exactly one of the iterms returned by the iterator.
pub trait FiniteSet:Set+IntoIterator<Item=Self>{
    const ORDER: usize;
}
pub trait Set: Clone+Eq {

}
/*impl<S,T> FiniteSubset<T> for S where T:FiniteSet, S:Subset<T> {
    const ORDER: usize = T::get_iter().filter(S::contains).collect::<Vec<_>>().len();
    type IterType = Filter<T::IterType,fn(&T)->bool>;
    fn get_iter() -> Self::IterType {
        T::get_iter().filter(S::contains)
    }
}*/
pub trait O2<S>{
    const F: fn(S,S)->S;
}
pub trait O0<S>{
    const F: fn()->S;
}
pub trait O1<S> {
    const F: fn(S)->S;
}
///Any implementation must guarantee that:
/// * the multiplication `Operation` is associative, and
/// * `Monoid::identity` returns the identity of the multiplication.
pub trait Monoid<Operation:O2<Self>>:Set{
    fn identity() -> Self;
    /// The correctness of the default implementation of this function relies on the properties
    /// required by this trait.
    fn pow(a:Self, n: u64)->Self {
        if n==0 {return Self::identity()}
        let rem=if n%2==1 {a.clone()} else {Self::identity()};
        let i=Self::pow(a,n/2);
        Operation::F(rem,Operation::F(i.clone(),i))
    }
    fn times(self, other:Self) -> Self{
        Operation::F(self,other)
    }
}

/// In addition to the properties required by the [`Monoid`] trait, any implementation must
/// guarantee that the product of any element with its inverse gives the identity.
pub trait Group<Operation:O2<Self>>:Monoid<Operation>{
    fn identity() -> Self {
        <Self as Monoid<Operation>>::identity()
    }
    /// For a implementation to be correct, for every element `a` in a group `G`, `Operation::F(a,
    /// a.inverse())` should be `==` to `G::identity()`.
    fn inverse(self)->Self;
    fn pow(a:Self, n: i64)->Self {
        if n>0 {return <Self as Monoid<Operation>>::pow(a, n as u64)}
        <Self as Monoid<Operation>>::pow(a,-n as u64).inverse()
    }
    fn times(self, other:Self) -> Self {
        Operation::F(self,other)
    }
}
pub trait RingOperations<T> where Self::PLUS:O2<T>,Self::TIMES:O2<T>{
    type PLUS;
    type TIMES;
}
pub trait Ring<O>:Group<O::PLUS>+Monoid<O::TIMES> where O:RingOperations<Self> {
    //Any implementation must guarantee that TIMES distributes over PLUS
    fn zero()->Self {
        <Self as Group<O::PLUS>>::identity()
    }
    fn one()->Self {
        <Self as Monoid<O::TIMES>>::identity()
    }
    fn negated(self)->Self {
        <Self as Group<O::PLUS>>::inverse(self)
    }
    fn plus(self, other:Self) -> Self {
        O::PLUS::F(self,other)
    }
    fn times(self, other:Self) -> Self {
        O::TIMES::F(self,other)
    }
    fn times_integer(self, other:i64) -> Self {
        Ring::times(self,Group::pow(Self::one(),other))
    }
}
pub trait Field<O>:Ring<O> where O:RingOperations<Self>, O::TIMES:O2<NonZero<Self,O>>, NonZero<Self,O>:Group<O::TIMES> {
    /// Multiplicative inverse. Panics if given zero.
    fn inverse(self) -> Self {
        <NonZero<Self,O> as Subset<Self>>::try_from(self).inverse().inclusion()
    }
}
struct Polynomial<R,O:RingOperations<R>> where R:Ring<O> {
    coefficients:Vec<R>,
    o:PhantomData<O>
}
#[derive(PartialEq,PartialOrd,Eq,Ord)]
pub enum Degree {
    NegInfty,
    Integer(usize)
}
impl Degree {
    /// Panics if the degree is `Degree::NegInfty`.
    fn unwrap(self)->usize {
        match self {
            Self::Integer(n)=>n,
            Self::NegInfty=>panic!()
        }
    }
}
impl PartialEq<usize> for Degree {
    fn eq(&self, other: &usize) -> bool {
        match *self {
            Degree::NegInfty => false,
            Degree::Integer(n) => &n==other
        }
    }
}
impl PartialOrd<usize> for Degree {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        match *self {
            Degree::NegInfty => Some(Ordering::Less),
            Degree::Integer(n) => n.partial_cmp(other)
        }
    }
}
impl Add<usize> for Degree {
    type Output = Degree;
    fn add(self, rhs: usize) -> Self::Output {
        match self {
            Degree::NegInfty => Degree::NegInfty,
            Degree::Integer(n) => Degree::Integer(n+rhs)
        }
    }
}
impl Add<Degree> for Degree {
    type Output = Degree;
    fn add(self, other:Degree) -> Degree {
        match self {
            Degree::NegInfty => Degree::NegInfty,
            Degree::Integer(n) => other+n
        }
    }
}
impl<R,O> Polynomial<R,O> where O:RingOperations<R>,R:Ring<O>{
    fn trim_zeros(mut v:Vec<R>)->Vec<R> {
        if !v.is_empty() {
            let mut k=v.pop().unwrap();
            while k==R::zero()&&!v.is_empty(){
                k=v.pop().unwrap();
            }
            if k!=R::zero() {
                v.push(k)
            }
        }
        v
    }
    fn zero()->Self {
        Polynomial {coefficients:vec![],o:PhantomData}}
    fn one()->Self {
        Polynomial {coefficients:vec![R::one()],o:PhantomData}}
    fn x()->Self {
        Polynomial {coefficients:vec![R::one(),R::zero()],o:PhantomData}
    }
    fn constant(c:R)->Self {
        Polynomial {coefficients:Self::trim_zeros(vec![c]),o:PhantomData}
    }
    fn degree(&self) -> Degree {
        if self.coefficients.is_empty() {Degree::NegInfty} else {Degree::Integer(self.coefficients.len()-1)}
    }
    fn coefficient(&self,n:usize) -> R{
        if self.degree()>=n {
            self.coefficients[n].clone()
        } else {
            R::zero()
        }
    }
    fn add(self, other:Self)->Self{
        let l=Degree::max(self.degree(),other.degree());
        let mut data=vec![];
        let mut n = 0;
        while l>=n{
            data.push(Ring::plus(self.coefficient(n),other.coefficient(n)));
            n+=1;
        }
        data=Self::trim_zeros(data);
        Polynomial {coefficients:data,o:PhantomData}
    }
    fn mul(self, other:Self) ->Self{
        let l = self.degree()+other.degree();
        let mut res=vec![];
        let mut i=0;
        while l>=i {
            let mut s=R::zero();
            for j in 0..=i {
                s=Ring::plus(s,Ring::times(self.coefficient(j),other.coefficient(i-j)));
            }
            res.push(s);
            i+=1;
        }
        res=Self::trim_zeros(res);
        Polynomial {coefficients:res,o:PhantomData}
    }
    
    fn negated(self) -> Self {
        let mut res=vec![];
        for i in 0..self.coefficients.len() {
            res.push(self.coefficients[i].clone().negated());
        }
        Polynomial {coefficients:res, o:PhantomData}
    }
    //fn derivative(self)->Self {
        //self
    //}
    fn of(&self,x:R) -> R {
        let mut n = 0;
        let mut res = R::zero();
        let mut xPower = R::one();
        while self.degree()>=n {
            res=res.plus(Ring::times(self.coefficient(n),xPower.clone()));
            xPower=Ring::times(xPower,x.clone());
            n+=1;
        }
        res
    }
    /// Gives the leading coefficient of self. The result is guaranteed to be nonzero. Panics if
    /// given the zero polynomial, which has no leading coefficient.
    fn lead_coeff(&self) -> R {
        match self.degree() {
            Degree::Integer(n) => self.coefficient(n),
            Degree::NegInfty => panic!()
        }
    }
}
impl<F,O> Polynomial<F,O> where O:RingOperations<F>,F:Field<O>,O::TIMES:O2<NonZero<F,O>>,NonZero<F,O>:Group<O::TIMES>{
    /// Panics if the divisor is zero
    fn divide(mut dividend:Self,divisor:Self) -> (Self, Self) {
        let n = divisor.degree().unwrap();
        let i = Field::inverse(divisor.lead_coeff());
        let mut quotient = Self::zero();
        while dividend.degree()>=divisor.degree() {
            let m=dividend.degree().unwrap();
            let term = Ring::times(Self::constant(Ring::times(dividend.lead_coeff(),i.clone())),Monoid::<PTIMES<F,O>>::pow(Self::x(),(m-n) as u64));
            quotient=quotient+term.clone();
            dividend=dividend+(divisor.clone()*term).negated();
        }
        (quotient, dividend)
    }
}
impl<R,O> Add<Polynomial<R,O>> for Polynomial<R,O> where O:RingOperations<R>,R:Ring<O> {
    type Output=Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.add(rhs)
    }
}
impl<R,O> Mul<Polynomial<R,O>> for Polynomial<R,O> where O:RingOperations<R>,R:Ring<O> {
    type Output=Self;
    fn mul(self, rhs: Polynomial<R,O>) -> Self::Output {
        self.mul(rhs)
    }
}
struct PPLUS<R,O> where O:RingOperations<R>,R:Ring<O> {
    r:PhantomData<R>,
    o:PhantomData<O>,
}
struct PTIMES<R,O> where O:RingOperations<R>,R:Ring<O> {
    r:PhantomData<R>,
    p:PhantomData<O>,
}
impl<R,O> O2<Polynomial<R,O>> for PPLUS<R,O> where O:RingOperations<R>,R:Ring<O>{
    const F: fn(Polynomial<R,O>,Polynomial<R,O>)->Polynomial<R,O> = <Polynomial<R,O>>::add;
}
impl<R,O> O2<Polynomial<R,O>> for PTIMES<R,O> where O:RingOperations<R>,R:Ring<O>{
    const F: fn(Polynomial<R,O>,Polynomial<R,O>)->Polynomial<R,O> = <Polynomial<R,O>>::mul;
}

impl<R,O> PartialEq for Polynomial<R,O> where O:RingOperations<R>,R:Ring<O> {
    fn eq(&self, other: &Self)->bool {
        self.coefficients==other.coefficients
    }
}
impl<R,O> Eq for Polynomial<R,O> where O:RingOperations<R>,R:Ring<O> {
}
impl<R,O> Clone for Polynomial<R,O> where O:RingOperations<R>,R:Ring<O> {
    fn clone(&self) -> Self {
        Polynomial {coefficients:self.coefficients.clone(),o:PhantomData}
    }
}
impl<R,O> Set for Polynomial<R,O> where O:RingOperations<R>,R:Ring<O> {

}
impl<R,O> Monoid<PTIMES<R,O>> for Polynomial<R,O> where O:RingOperations<R>,R:Ring<O> {
    fn identity() -> Self {
        Self::one()
    }
}
impl<R,O> Monoid<PPLUS<R,O>> for Polynomial<R,O> where O:RingOperations<R>,R:Ring<O> {
    fn identity() -> Self {
        Self::zero()
    }
}
impl<R,O> Group<PPLUS<R,O>> for Polynomial<R,O> where O:RingOperations<R>,R:Ring<O> {
    fn inverse(self) -> Self {
        self.negated()
    }
}
struct PolyOps<R,O> where O:RingOperations<R>,R:Ring<O> {
    r:PhantomData<R>,
    o:PhantomData<O>
}
impl<R,O> RingOperations<Polynomial<R,O>> for PolyOps<R,O> where O:RingOperations<R>, R:Ring<O> {
    type PLUS = PPLUS<R,O>;
    type TIMES = PTIMES<R,O>;
}
impl<R,O> Ring<PolyOps<R,O>> for Polynomial<R,O> where O:RingOperations<R>,R:Ring<O> {

}
/*struct ProductTimes<M1,T1:O2<M1>,M2,T2:O2<M2>> where M1:Monoid<T1>, M2:Monoid<T2> {
    g1:PhantomData<M1>,
    t1:PhantomData<T1>,
    g2:PhantomData<M2>,
    t2:PhantomData<T2>
}*/
impl<M1,T1:O2<M1>,M2,T2:O2<M2>> O2<(M1,M2)> for (T1,T2)  where M1:Monoid<T1>, M2:Monoid<T2> {
    const F:fn((M1,M2),(M1,M2))->(M1,M2) = |(m1,m2),(m3,m4)| (m1.times(m3),m2.times(m4));
}
impl<M1,T1:O2<M1>,M2,T2:O2<M2>> Monoid<(T1,T2)> for (M1,M2) where M1:Monoid<T1>, M2:Monoid<T2> {
    fn identity() -> Self {
        (M1::identity(),M2::identity())
    }
}
impl<M1,T1:O2<M1>,M2,T2:O2<M2>> Group<(T1,T2)> for (M1,M2) where M1:Group<T1>, M2:Group<T2> {
    fn inverse(self)->Self {
        (self.0.inverse(),self.1.inverse())
    }
}
/*struct ProductOps<R,O,S,P> where O:RingOperations<R>,R:Ring<O>, P:RingOperations<S>,S:Ring<P> {
    r:PhantomData<R>,
    o:PhantomData<O>,
    s:PhantomData<S>,
    p:PhantomData<P>
}*/
impl<R,O,S,P> RingOperations<(R,S)> for (O,P) where O:RingOperations<R>,R:Ring<O>, P:RingOperations<S>,S:Ring<P> {
    type PLUS = (O::PLUS,P::PLUS);
    type TIMES = (O::TIMES,P::TIMES);
}
impl<R,O,S,P> Ring<(O,P)> for (R,S) where O:RingOperations<R>,R:Ring<O>, P:RingOperations<S>,S:Ring<P> {
}
pub struct NonZero<R,O> where R:Ring<O>,O:RingOperations<R> {
    r:R,
    o:PhantomData<O>
}

impl<R,O> Subset<R> for NonZero<R,O> where R:Ring<O>, O:RingOperations<R> {
    fn contains(r:&R) -> bool {
        r==&R::zero()
    }
    fn inclusion(self) -> R {
        self.r
    }
    fn try_from(t:R) -> Self {
        if Self::contains(&t) {
            Self {r:t, o:PhantomData}
        } else {
            panic!()
        }
    }
}
impl<S,T> Set for (S,T) where S:Set, T:Set {}
impl Set for i64 {}
struct i64Plus {}
impl O2<i64> for i64Plus {
    const F: fn(i64,i64)->i64 = |a,b|a+b;
}
impl Monoid<i64Plus> for i64 {
    fn identity() -> Self {
        0
    }
}
impl Group<i64Plus> for i64 {
    fn inverse(self)->Self {
        -self
    }
}
struct i64Times {}
impl O2<i64> for i64Times {
    const F: fn(i64,i64)->i64 = |a,b|a*b;
}
impl Monoid<i64Times> for i64 {
    fn identity() -> Self {
        1
    }
}
struct i64Ops {}
impl RingOperations<i64> for i64Ops {
    type PLUS = i64Plus;
    type TIMES = i64Times;
}
impl Ring<i64Ops> for i64 {}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn t1() {
        {
            let x=(1i64,4i64);
            let y=(-2i64,3i64);
            let f=Polynomial{coefficients:vec![(1,0),(2,1),(-1,3)],o:PhantomData};
            println!("{:?}",x.plus(y));
            println!("{:?}",Ring::times(x,y));
            println!("{:?}",f.of(x))
        };
    }
}