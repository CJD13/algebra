mod nonzero;
mod operation;
mod polynomial;
mod quotient;
mod set;
mod structure;
mod modular;
mod wrapper;
mod test;
mod unit;
mod impls;
extern crate take_mut;
extern crate num_bigint;
#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use crate::{
        polynomial::Polynomial,
        quotient::{ QuotientGroup, QuotientRing, IntMultiples},
        structure::{ring::{Ring}, group::Group, monoid::Monoid, euclidean_ring::EuclideanRing},
        set::{Subset}, impls::StandardOps,
        impls
    };

    #[test]
    fn t1() {
        {
            let x = (1i64, 4i64);
            let y = (-2i64, 3i64);
            let f = Polynomial {
                coefficients: vec![(1, 0), (2, 1), (-1, 3)],
                o: PhantomData,
            };
            println!("{:?}", x.plus(&y));
            println!("{:?}", x.times(&y));
            println!("{:?}", f.of(x));
            type ZMod15 = QuotientRing<i64,StandardOps,IntMultiples<15>>;
            let o = ZMod15::from(27);
            println!("{}",IntMultiples::<15>::contains(&30));
            let p = ZMod15::from(2);
            println!("{:?}", Ring::pow(o.clone(), 20));
            println!("{:?}",o);
            println!("{:?}", Ring::pow(p,15));
            type ZMod97 = QuotientRing<i64,StandardOps,IntMultiples<97>>;
            let two = ZMod97::from(2);
            let three = ZMod97::from(3);
            println!("{:?}", Ring::pow(two.clone(), 16));
            println!("{:?}", Ring::pow(two.clone(), 24));
            println!("{:?}", Ring::pow(two, 48));
            //order of 2 is 48
            println!("{:?}", Ring::pow(three,48));
            //3 is a square mod 97
            println!("{:?}",(-5i64).div_euclid(-3))
        };
    }
}
