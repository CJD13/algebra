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
        quotient::{Multiples, QuotientGroup},
        structure::{ring::Ring, group::Group},
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
            type ZMod15 = QuotientGroup<i64, Multiples<15>, i64Plus>;
            let o = ZMod15::from(27);
            println!("{:?}", Group::pow(o.clone(), 15));
            println!("{:?}", Group::pow(o, 7));
        };
    }
}
