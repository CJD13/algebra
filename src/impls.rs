use std::ops::{Mul, Add, Neg};

use num_bigint::BigInt;
use crate::{structure::{ring::{RingOperations, Ring}, monoid::Monoid, group::Group}, operation::O2, set::Set};


pub struct Plus {}
trait StandardRing: for<'a> Add<&'a Self, Output=Self>+for<'a> Mul<&'a Self,Output=Self>+Clone+Eq+From<i8>+Neg<Output=Self>{}
impl StandardRing for i8{}
impl StandardRing for i16{}
impl StandardRing for i32{}
impl StandardRing for i64{}
impl StandardRing for BigInt{}
impl<R:StandardRing> Set for R{}
impl<R:StandardRing> O2<R> for Plus {
    const F: fn(R, &R) -> R = |a, b| a + b;
}
pub struct Times {}
impl<R:StandardRing> O2<R> for Times {
    const F: fn(R, &R) -> R = |a, b| a * b;
}
pub struct StandardOps {}
impl<R:StandardRing> RingOperations<R> for StandardOps {
    type PLUS = Plus;
    type TIMES = Times;
}
impl<R:StandardRing> Ring<StandardOps> for R {}

impl<R:StandardRing> Monoid<Plus> for R {
    fn identity() -> Self {
        R::from(0)
    }
}
impl<R:StandardRing> Monoid<Times> for R {
    fn identity() -> Self {
        R::from(1)
    }
}
impl<R:StandardRing> Group<Plus> for R {
    fn inverse(self) -> Self {
        -self
    }
}
