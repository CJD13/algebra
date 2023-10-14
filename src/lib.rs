mod nonzero;
mod operation;
mod polynomial;
mod quotient;
mod set;
mod structure;

mod test;

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use crate::{
        operation::i64Plus,
        polynomial::Polynomial,
        quotient::{Multiples, QuotientGroup, QuotientRing},
        structure::{ring::{Ring, i64Ops}, group::Group, monoid::Monoid},
        set::{Subset},
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
            println!("{:?}", x.plus(y));
            println!("{:?}", Ring::times(x, y));
            println!("{:?}", f.of(x));
            type ZMod15 = QuotientRing<i64,i64Ops,Multiples<15>>;
            let o = ZMod15::from(27);
            println!("{}",Multiples::<15>::contains(&30));
            let p = ZMod15::from(2);
            println!("{:?}", Ring::pow(o.clone(), 20));
            println!("{:?}",o);
            println!("{:?}", Ring::pow(p,15));
        };
    }
}
