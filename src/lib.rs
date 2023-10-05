use std::ops::Mul;
use std::marker::PhantomData;
pub trait FiniteSet:Set+IntoIterator {
    const ORDER: u64;
    //A struct implementing FiniteSet must have ORDER equal to the number of objects of that struct.
    //It must also be able to provide an iterator over its elements
}
pub trait Set: Clone+PartialEq {

}
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
pub trait Ring<PLUS:O2<Self>,TIMES:O2<Self>>:Group<PLUS>+Monoid<TIMES>{
    //Any implementation must guarantee that TIMES distributes over PLUS
    fn zero()->Self {
        <Self as Group<PLUS>>::identity()
    }
    fn one()->Self {
        <Self as Monoid<TIMES>>::identity()
    }
    fn negated(self)->Self {
        <Self as Group<PLUS>>::inverse(self)
    }
    fn plus(self, other:Self) -> Self {
        PLUS::F(self,other)
    }
    fn times(self, other:Self) -> Self {
        TIMES::F(self,other)
    }
    fn times_integer(self, other:i64) -> Self {
        Ring::times(self,Group::pow(Self::one(),other))
    }
}
struct Polynomial<R,PLUS:O2<R>,TIMES:O2<R>> where R:Ring<PLUS,TIMES> {
    data:Vec<R>,
    p:PhantomData<PLUS>,
    t:PhantomData<TIMES>
}

impl<R,PLUS:O2<R>,TIMES:O2<R>> Polynomial<R,PLUS,TIMES> where R:Ring<PLUS,TIMES>{
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
        Polynomial {data:vec![],p:PhantomData,t:PhantomData}}
    fn one()->Self {
        Polynomial {data:vec![<R as Monoid<TIMES>>::identity()],p:PhantomData,t:PhantomData}}
    fn degree(&self) -> Option<usize> {
        if self.data.len()==0 {None} else {Some(self.data.len()-1)}
    }
    fn coefficient(&self,n:usize) -> R{
        if self.degree().is_some_and(|d| n<=d) {
            self.data[n].clone()
        } else {
            R::zero()
        }
    }
    fn add(self, other:Self)->Self{
        let l=usize::max(self.degree().map_or(0,|x|x+1),other.degree().map_or(0,|x|x+1));
        let mut data=vec![];
        for n in 0..l{
            data.push(PLUS::F(self.coefficient(n),other.coefficient(n)));
        }
        data=Self::trim_zeros(data);
        Polynomial {data:data,p:PhantomData,t:PhantomData}
    }
    fn mul(self, other:Self) ->Self{
        let l = self.degree().and_then(|x| other.degree().and_then(|y| Some(x+y)));
        l.map_or(Self::zero(),
            |l| {
                let mut res=vec![];
                for i in 0..=l {
                    let mut s=R::zero();
                    for j in 0..=i {
                        s=PLUS::F(s,TIMES::F(self.coefficient(j),other.coefficient(i-j)));
                    }
                    res.push(s);
                }
                res=Self::trim_zeros(res);
                Polynomial {data:res,p:PhantomData,t:PhantomData}
        })
    }
    fn negated(self) -> Self {
        let mut res=vec![];
        for i in 0..self.data.len() {
            res.push(self.data[i].clone().negated());
        }
        Polynomial {data:res, p:PhantomData, t:PhantomData}
    }
    //fn derivative(self)->Self {
        //self
    //}
}
struct PPLUS<R,PLUS:O2<R>,TIMES:O2<R>> where R:Ring<PLUS,TIMES> {
    r:PhantomData<R>,
    p:PhantomData<PLUS>,
    t:PhantomData<TIMES>
}
struct PTIMES<R,PLUS:O2<R>,TIMES:O2<R>> where R:Ring<PLUS,TIMES> {
    r:PhantomData<R>,
    p:PhantomData<PLUS>,
    t:PhantomData<TIMES>
}
impl<R,PLUS:O2<R>,TIMES:O2<R>> O2<Polynomial<R,PLUS,TIMES>> for PPLUS<R,PLUS,TIMES> where R:Ring<PLUS,TIMES>{
    const F: fn(Polynomial<R,PLUS,TIMES>,Polynomial<R,PLUS,TIMES>)->Polynomial<R,PLUS,TIMES> = <Polynomial<R,PLUS,TIMES>>::add;
}
impl<R,PLUS:O2<R>,TIMES:O2<R>> O2<Polynomial<R,PLUS,TIMES>> for PTIMES<R,PLUS,TIMES> where R:Ring<PLUS,TIMES>{
    const F: fn(Polynomial<R,PLUS,TIMES>,Polynomial<R,PLUS,TIMES>)->Polynomial<R,PLUS,TIMES> = <Polynomial<R,PLUS,TIMES>>::mul;
}

impl<R,PLUS:O2<R>,TIMES:O2<R>> PartialEq for Polynomial<R,PLUS,TIMES> where R:Ring<PLUS,TIMES> {
    fn eq(&self, other: &Self)->bool {
        return self.data==other.data
    }
}
impl<R,PLUS:O2<R>,TIMES:O2<R>> Clone for Polynomial<R,PLUS,TIMES> where R:Ring<PLUS,TIMES> {
    fn clone(&self) -> Self {
        Polynomial {data:self.data.clone(),p:PhantomData,t:PhantomData}
    }
}
impl<R,PLUS:O2<R>,TIMES:O2<R>> Set for Polynomial<R,PLUS,TIMES> where R:Ring<PLUS,TIMES> {

}
impl<R,PLUS:O2<R>,TIMES:O2<R>> Monoid<PTIMES<R,PLUS,TIMES>> for Polynomial<R,PLUS,TIMES> where R:Ring<PLUS,TIMES> {
    fn identity() -> Self {
        Self::one()
    }
}
impl<R,PLUS:O2<R>,TIMES:O2<R>> Monoid<PPLUS<R,PLUS,TIMES>> for Polynomial<R,PLUS,TIMES> where R:Ring<PLUS,TIMES> {
    fn identity() -> Self {
        Self::zero()
    }
}
impl<R,PLUS:O2<R>,TIMES:O2<R>> Group<PPLUS<R,PLUS,TIMES>> for Polynomial<R,PLUS,TIMES> where R:Ring<PLUS,TIMES> {
    fn inverse(self) -> Self {
        self.negated()
    }
}
impl<R,PLUS:O2<R>,TIMES:O2<R>> Ring<PPLUS<R,PLUS,TIMES>,PTIMES<R,PLUS,TIMES>> for Polynomial<R,PLUS,TIMES> where R:Ring<PLUS,TIMES> {

}
struct ProductTimes<M1,T1:O2<M1>,M2,T2:O2<M2>> where M1:Monoid<T1>, M2:Monoid<T2> {
    g1:PhantomData<M1>,
    t1:PhantomData<T1>,
    g2:PhantomData<M2>,
    t2:PhantomData<T2>
}
impl<M1,T1:O2<M1>,M2,T2:O2<M2>> O2<(M1,M2)> for ProductTimes<M1,T1,M2,T2>  where M1:Monoid<T1>, M2:Monoid<T2> {
    const F:fn((M1,M2),(M1,M2))->(M1,M2) = |(m1,m2),(m3,m4)| (m1.times(m3),m2.times(m4));
}
impl<M1,T1:O2<M1>,M2,T2:O2<M2>> Monoid<ProductTimes<M1,T1,M2,T2>> for (M1,M2) where M1:Monoid<T1>, M2:Monoid<T2> {
    fn identity() -> Self {
        (M1::identity(),M2::identity())
    }
}
impl<M1,T1:O2<M1>,M2,T2:O2<M2>> Group<ProductTimes<M1,T1,M2,T2>> for (M1,M2) where M1:Group<T1>, M2:Group<T2> {
    fn inverse(self)->Self {
        (self.0.inverse(),self.1.inverse())
    }
}
impl<M1,P1:O2<M1>,T1:O2<M1>,M2,P2:O2<M2>,T2:O2<M2>> Ring<ProductTimes<M1,P1,M2,P2>,ProductTimes<M1,T1,M2,T2>> for (M1,M2) where M1:Ring<P1,T1>, M2:Ring<P2,T2> {
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
impl Ring<i64Plus,i64Times> for i64 {}
fn main() {
    let x=(1i64,4i64);
    let y=(-2i64,3i64);

    println!("{:?}",x.plus(y));
    println!("{:?}",Ring::times(x,y));
}
#[cfg(test)]
mod tests {
    use crate::main;
    #[test]
    fn t1() {
        main();
    }
}