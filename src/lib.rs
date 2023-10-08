use core::panic;
use std::cmp::Ordering;
use std::iter::Filter;
use std::ops::{Mul, Add};
use std::marker::PhantomData;
//use num::BigInt;
mod test;
pub trait Subset<T> where T:Set {
    //A subset of T is determined by which elements of T are members of it.
    fn contains(t:&T) -> bool;
    //any object of type Self can be coerced into one of type T
    fn inclusion(self) -> T;
    //some Ts can be turned into Selfs
    fn try_into(t:T) -> Self;
}
pub trait FiniteSubset<T>:Subset<T>+IntoIterator<Item=T> where T:Set{
    const ORDER: usize;
    //A finite subset can be iterated over.
    //An implementation must guarantee that each member t of T
    //is == to exactly one item of the iterator if Self::contains(t) is true
    //and does not == any if it is false.
}
pub trait FiniteSet:Set+IntoIterator<Item=Self>{
    const ORDER: usize;
    //A struct implementing FiniteSet must have ORDER equal to the number of objects of that struct.
    //It must also be able to provide an iterator over its elements.
    //Any member of the struct must be == to exactly one of the iterms returned by the iterator.
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
pub trait Monoid<Operation:O2<Self>>:Set{
    fn identity() -> Self;
    //Any implementation must guarantee that the multiplication is associative,
    //and that identity returns the identity of the multiplication.
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

pub trait Group<Operation:O2<Self>>:Monoid<Operation>{
    fn identity() -> Self {
        <Self as Monoid<Operation>>::identity()
    }
    fn inverse(self)->Self;
    //Any implementation must guarantee that
    //the product of any element with its inverse gives the identity.
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
struct Polynomial<R,O:RingOperations<R>> where R:Ring<O> {
    coefficients:Vec<R>,
    o:PhantomData<O>
}
#[derive(PartialEq,PartialOrd,Eq,Ord)]
pub enum Degree {
    NegInfty,
    Integer(usize)
}
impl PartialEq<usize> for Degree {
    fn eq(&self, other: &usize) -> bool {
        match self {
            &Degree::NegInfty => false,
            &Degree::Integer(n) => &n==other
        }
    }
}
impl PartialOrd<usize> for Degree {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        match self {
            &Degree::NegInfty => Some(Ordering::Less),
            &Degree::Integer(n) => n.partial_cmp(other)
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
        if v.len()>0 {
            let mut k=v.pop().unwrap();
            while k==R::zero()&&v.len()>0{
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
        Polynomial {coefficients:vec![<R as Monoid<O::TIMES>>::identity()],o:PhantomData}}
    fn degree(&self) -> Degree {
        if self.coefficients.len()==0 {Degree::NegInfty} else {Degree::Integer(self.coefficients.len()-1)}
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
        return self.coefficients==other.coefficients
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
struct NonZero<R,O> where R:Ring<O>,O:RingOperations<R> {
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
    fn try_into(t:R) -> Self {
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
fn main() {
    let x=(1i64,4i64);
    let y=(-2i64,3i64);
    let f=Polynomial{coefficients:vec![(1,0),(2,1),(-1,3)],o:PhantomData};
    println!("{:?}",x.plus(y));
    println!("{:?}",Ring::times(x,y));
    println!("{:?}",f.of(x))
}
#[cfg(test)]
mod tests {
    use crate::main;
    #[test]
    fn t1() {
        main();
    }
}